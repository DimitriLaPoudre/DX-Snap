use crate::protocol::ClientState;

use tokio::io::Result;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct Client {
    pub reader: OwnedReadHalf,
    pub writer: OwnedWriteHalf,
    pub state: ClientState,
}

impl Client {
    pub async fn create(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();
        Client {
            reader,
            writer,
            state: ClientState::Handshake,
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
