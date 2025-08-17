use crate::Client;

use async_trait::async_trait;
use std::io::{Error, ErrorKind};
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

#[derive(Copy, Clone)]
pub enum ClientState {
    Unknown,
    Connected,
}

impl ClientState {
    pub async fn send(&self, client: &mut Client) -> Result<()> {
        self.behavior().send(client).await
    }

    pub async fn received(&self, client: &mut Client) -> Result<()> {
        self.behavior().received(client).await
    }

    #[inline(always)]
    fn behavior(&self) -> &'static dyn CommandBehavior {
        match self {
            ClientState::Unknown => &UNKNOWN_BEHAVIOR,
            ClientState::Connected => &CONNECTED_BEHAVIOR,
        }
    }
}

#[async_trait]
trait CommandBehavior {
    async fn send(&self, client: &mut Client) -> Result<()>;
    async fn received(&self, client: &mut Client) -> Result<()>;
}

struct UnknownBehavior;
static UNKNOWN_BEHAVIOR: UnknownBehavior = UnknownBehavior;

#[async_trait]
impl CommandBehavior for UnknownBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client) -> Result<()> {
        let mut buf = [0; 256];
        let size_read = client.reader.read(&mut buf).await?;
        if size_read == 0 {
            return Err(Error::new(
                ErrorKind::ConnectionAborted,
                "Connection closed",
            ));
        }

        client.writer.write("Ok\n".as_bytes()).await?;

        Ok(())
    }
}

struct ConnectedBehavior;
static CONNECTED_BEHAVIOR: ConnectedBehavior = ConnectedBehavior;

#[async_trait]
impl CommandBehavior for ConnectedBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client) -> Result<()> {
        let mut buf = [0; 256];
        let size_read = client.reader.read(&mut buf).await?;
        if size_read == 0 {
            return Err(Error::new(
                ErrorKind::ConnectionAborted,
                "Connection closed",
            ));
        }

        client.writer.write("Ok\n".as_bytes()).await?;

        Ok(())
    }
}
