mod client;
mod prelude;
mod protocol;
mod states {
    pub mod homepage;
    pub mod login;
}

use crate::client::Client;
use crate::prelude::*;

use chrono::Local;
use fern::Dispatch;
use log::LevelFilter;
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::signal;
// use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Notify;
use tokio::task::JoinSet;

fn setup_logger() -> Result<()> {
    let path = dirs::home_dir()
        .ok_or(anyhow!("home directory can't be find"))?
        .join(".dx_snap")
        .join("logs")
        .join(format!(
            "logs_{}.log",
            Local::now().format("%Y-%m-%d_%H:%M:%S"),
        ));
    std::fs::create_dir_all(dirs::home_dir().unwrap().join(".dx_snap").join("logs"))?;

    let file_config = Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(fern::log_file(path)?);

    let stdout_config = Dispatch::new()
        .level(if cfg!(debug_assertions) {
            LevelFilter::Trace
        } else {
            LevelFilter::Info
        })
        .chain(std::io::stdout());

    let base_config = Dispatch::new().format(|out, message, record| {
        out.finish(format_args!(
            "[{}][{}][{}:{}] {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.module_path().unwrap_or("<?>"),
            record.line().unwrap_or(0),
            message
        ))
    });

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;

    Ok(())
}

struct Server {
    notify: Arc<Notify>,
    sql_pool: Arc<Pool<Postgres>>,
    listener: TcpListener,
    join_set: JoinSet<Result<()>>,
}

impl Server {
    async fn new(addr: &str) -> Self {
        let notify = Arc::new(Notify::new());
        let listener = TcpListener::bind(addr).await.unwrap();
        let mut join_set = JoinSet::new();
        let sql_pool = Arc::new(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(100)
                .connect("postgres://dx_snap_server:dx_snap_on_top@localhost/dx_snap_db")
                .await
                .unwrap(),
        );

        let tmp_notify = Arc::clone(&notify);
        join_set.spawn(async move {
            tmp_notify.notified().await;
            Ok(())
        });

        log::info!("Server up...");

        Self {
            notify,
            sql_pool,
            listener,
            join_set,
        }
    }

    async fn handle_connexion(&mut self, (stream, _addr): (TcpStream, SocketAddr)) {
        let notify = Arc::clone(&self.notify);
        let sql_pool = Arc::clone(&self.sql_pool);
        self.join_set.spawn(async move {
            let mut client: Client;
            tokio::select! {
                _ = notify.notified() => {
                    log::info!("shell(): stopped by notify");
                    return Ok(());
                }
                ret = Client::create(stream, sql_pool) => {
                    match ret {
                        Ok(value) => client = value,
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }
            tokio::select! {
                _ = notify.notified() => {
                    log::info!("shell(): stopped by notify");
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
    setup_logger().unwrap();

    const ADDR: &str = "0.0.0.0:13216";

    let mut server = Server::new(ADDR).await;

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                log::info!("Ctrl+C received, notify send");
                server.notify.notify_waiters();
            }
            ret = server.listener.accept() => {
                match ret {
                    Ok(value) => server.handle_connexion(value).await,
                    Err(e) => log::error!("accept(): {e}"),
                }
            }
            ret = server.join_set.join_next() => {
                 match ret {
                    Some(Ok(Ok(()))) => {}
                    Some(Ok(Err(e))) => log::error!("join_next(): {e}"),
                    Some(Err(e)) => log::error!("join_next(): {e}"),
                    None => {
                        log::info!("No more task, will quit safely.");
                        break;
                    }
                }
            }
        }
    }
}
