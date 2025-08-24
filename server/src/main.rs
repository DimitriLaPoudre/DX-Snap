mod client;
mod protocol;

use crate::client::Client;

use chrono::{DateTime, Utc};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::Result;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::{Mutex, Notify, RwLock};
use tokio::task::JoinSet;

struct Server {
    notify: Arc<Notify>,
    logs: BufWriter<File>,
    listener: TcpListener,
    join_set: JoinSet<Result<()>>,
    clients: Arc<RwLock<Vec<String>>>,
}

impl Server {
    async fn new(addr: &str) -> Self {
        let notify = Arc::new(Notify::new());
        let path = dirs::home_dir().unwrap().join(".dx_snap").join("logs.log");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .unwrap();
        let logs = BufWriter::new(file);
        let listener = TcpListener::bind(addr).await.unwrap();
        let mut join_set = JoinSet::new();
        let tmp_notify = Arc::clone(&notify);
        join_set.spawn(async move {
            tmp_notify.notified().await;
            Ok(())
        });
        let clients: Arc<RwLock<Vec<String>>> = Arc::new(RwLock::new(vec![]));

        Self {
            notify,
            logs,
            listener,
            join_set,
            clients,
        }
    }

    async fn handle_connexion(&mut self, (stream, addr): (TcpStream, SocketAddr)) {
        let notify = Arc::clone(&self.notify);
        let mut client = Client::create(stream).await;
        self.join_set.spawn(async move {
            tokio::select! {
                _ = notify.notified() => {
                    eprintln!("{{Unknown}}: shell() stopped by notify.");
                    Ok(())
                }
                ret = client.shell() => {
                    ret
                }
            }
        });
    }
}

#[tokio::main]
async fn main() {
    const ADDR: &str = "0.0.0.0:13216";
    let mut server = Server::new(ADDR).await;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                eprintln!("Ctrl+C received, notify send");
                server.notify.notify_waiters();
            }
            ret = server.listener.accept() => {
                match ret {
                    Ok(value) => server.handle_connexion(value).await,
                    Err(e) => eprintln!("accept() error received: {}", e),
                }
            }
            ret = server.join_set.join_next() => {
                 match ret {
                    Some(Ok(Ok(()))) => {}
                    Some(Ok(Err(e))) => eprintln!("join_next() error received: {}", e),
                    Some(Err(e)) => eprintln!("join_next() panic/abort received: {}", e),
                    None => {
                        eprintln!("No more task, can quit safely.");
                        break;
                    }
                }
            }
        }
    }
}
