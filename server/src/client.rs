use crate::prelude::*;
use crate::protocol::ClientState;

use sqlx::{Pool, Postgres};
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct Client {
    pub reader: OwnedReadHalf,
    pub writer: OwnedWriteHalf,
    pub sql_pool: Arc<Pool<Postgres>>,
    pub state: ClientState,
}

impl Client {
    pub async fn create(stream: TcpStream, sql_pool: Arc<Pool<Postgres>>) -> Self {
        let (reader, writer) = stream.into_split();
        Client {
            reader,
            writer,
            sql_pool,
            state: ClientState::Login,
        }
    }

    pub async fn shell(&mut self) -> Result<()> {
        loop {
            self.reader.readable().await?;

            let state = self.state;
            state.received(self).await?;
        }
    }
}
