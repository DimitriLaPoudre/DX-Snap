use crate::prelude::*;
use crate::protocol::ClientState;

use futures_util::stream::{SplitSink, SplitStream, StreamExt};
use sqlx::{Pool, Postgres};
use tokio::net::TcpStream;
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::protocol::Message};

pub struct Client {
    pub writer: SplitSink<WebSocketStream<TcpStream>, Message>,
    pub reader: SplitStream<WebSocketStream<TcpStream>>,
    pub sql_pool: Arc<Pool<Postgres>>,
    pub state: ClientState,
    pub id: i64,
}

impl Client {
    pub async fn create(stream: TcpStream, sql_pool: Arc<Pool<Postgres>>) -> Result<Self> {
        log::info!(
            "New client created from: {}.",
            match stream.peer_addr() {
                Ok(addr) => {
                    addr.ip().to_string()
                }
                Err(_) => {
                    "<Unknown-IP>".into()
                }
            }
        );

        let stream = accept_async(stream).await?;

        let (writer, reader) = stream.split();

        Ok(Client {
            writer,
            reader,
            sql_pool,
            state: ClientState::Login,
            id: 0,
        })
    }

    pub async fn shell(&mut self) -> Result<()> {
        loop {
            let state = self.state;
            state.received(self).await?;
        }
    }
}
