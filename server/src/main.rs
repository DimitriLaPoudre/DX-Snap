mod command_module;

use crate::command_module::ClientState;

use chrono::{DateTime, Utc};
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::io::Result;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, Notify, RwLock, mpsc};
use tokio::task::JoinSet;

type Username = String;
type Passwd = usize;

enum Request {
    Connexion(TcpStream),
}

struct Server {
    notify: Arc<Notify>,
    join_set: JoinSet<Result<()>>,
    clients: Arc<RwLock<Vec<(Username, Passwd)>>>,
    rx_connection: Receiver<Request>,
}

impl Server {
    fn new() -> (Self, Sender<Request>) {
        let notify = Arc::new(Notify::new());
        let join_set = JoinSet::new();
        let clients: Arc<RwLock<Vec<(Username, Passwd)>>> = Arc::new(RwLock::new(vec![]));
        let (tx, rx_connection) = mpsc::channel::<Request>(100);
        (
            Self {
                notify,
                join_set,
                clients,
                rx_connection,
            },
            tx,
        )
    }

    async fn handle_connexion(addr: &str, tx: Sender<Request>) -> Result<()> {
        let listener = TcpListener::bind(addr).await?;
        loop {
            let stream = match listener.accept().await {
                Ok(stream) => {
                    println!("New Client connected: {}", stream.1);
                    stream.0
                }

                Err(e) => {
                    eprintln!("[E] accept() -> (TcpStream, SocketAddr): {}", e);
                    continue;
                }
            };
            if let Err(_) = tx.send(Request::Connexion(stream)).await {
                return Ok(());
            }
        }
    }

    async fn add_user(&self, username: Username, passwd: Passwd) -> Result<()> {
        let mut write_guard = self.clients.write().await;
        if write_guard.iter().any(|(key, _)| *key == username) {
            Err(Error::new(
                ErrorKind::ConnectionAborted,
                "Username already exist",
            ))
        } else {
            write_guard.push((username, passwd));
            Ok(())
        }
    }
}

struct Client {
    reader: OwnedReadHalf,
    writer: OwnedWriteHalf,
    state: ClientState,
    username: String,
}

impl Client {
    async fn create(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        Client {
            reader,
            writer,
            state: ClientState::Unknown,
            username: String::new(),
        }
    }

    async fn shell(&mut self) -> Result<()> {
        loop {
            println!("test");
            self.reader.readable().await?;

            let state = self.state;
            state.received(self).await?;
        }
    }
}

#[tokio::main]
async fn main() {
    let (mut server, tx_connection) = Server::new();
    const ADDR: &str = "127.0.0.1:13216";

    {
        let notify = Arc::clone(&server.notify);
        server.join_set.spawn(async move {
            tokio::select! {
                _ = notify.notified() => {
                    eprintln!("handle_connexion() stopped by notify.");
                    Ok(())
                }
                ret = Server::handle_connexion(ADDR, tx_connection) => {
                    ret
                }
            }
        });
    }
    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                eprintln!("Ctrl+C received, notify send");
                server.notify.notify_waiters();
            }
            ret = server.rx_connection.recv() => {
                match ret {
                    Some(Request::Connexion(stream)) => {
                        let notify = Arc::clone(&server.notify);
                        let mut client = Client::create(stream).await;
                        server.join_set.spawn(async move {
                            tokio::select! {
                                _ = notify.notified() => {
                                    if client.username.is_empty() {
                                        eprintln!("{{Unknown}}: shell() stopped by notify.");
                                    } else {
                                        eprintln!("[{}]: shell() stopped by notify.", client.username);
                                    }
                                    Ok(())
                                }
                                ret = client.shell() => {
                                    ret
                                }
                            }
                        });
                    }
                    None => {}
                }
            }
            ret = server.join_set.join_next() => {
                    match ret {
                        Some(Ok(Ok(()))) => {}
                        Some(Ok(Err(e))) => {
                            eprintln!("join_next() error received: {}", e);
                        }
                        Some(Err(e)) => {
                            eprintln!("join_next() panic/abort received: {}", e);
                        }
                        None => {
                            eprintln!("No more task, can quit safely.");
                            break;
                        }
                    }
                }
        }
    }
}
