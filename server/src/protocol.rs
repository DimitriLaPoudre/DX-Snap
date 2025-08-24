use crate::client::Client;

use serde::{Deserialize, Serialize};

use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

#[derive(Copy, Clone)]
pub enum ClientState {
    Handshake,
    Connected,
}

#[derive(Deserialize, Serialize)]
pub struct Message {
    seq: u32,
    #[serde(flatten)]
    payload: MessagePayload,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
enum MessagePayload {
    Handshake(HandshakePayload),
}

impl ClientState {
    pub async fn send(&self, client: &mut Client) -> Result<()> {
        self.behavior().send(client).await
    }

    pub async fn received(&self, client: &mut Client) -> Result<()> {
        let mut buf_header = [0; 4];

        client.reader.read_exact(&mut buf_header).await?;
        let msg_length = u32::from_be_bytes(buf_header) as usize;

        let mut msg = vec![0; msg_length];
        client.reader.read_exact(&mut msg).await?;

        let msg: Message = serde_json::from_slice(&msg)?;

        self.behavior().received(client, msg).await
    }

    #[inline(always)]
    fn behavior(&self) -> &'static dyn CommandBehavior {
        match self {
            ClientState::Handshake => &HANDSHAKE_BEHAVIOR,
            ClientState::Connected => &CONNECTED_BEHAVIOR,
        }
    }
}

#[async_trait]
trait CommandBehavior {
    async fn send(&self, client: &mut Client) -> Result<()>;
    async fn received(&self, client: &mut Client, msg: Message) -> Result<()>;
}

static HANDSHAKE_BEHAVIOR: HandshakeBehavior = HandshakeBehavior;

struct HandshakeBehavior;

#[derive(Deserialize, Serialize)]
enum HandshakePayload {
    Token { token: [u8; 32] },
    Connect { username: String, password: String },
    Create { username: String, password: String },
}

#[async_trait]
impl CommandBehavior for HandshakeBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client, msg: Message) -> Result<()> {
        if let MessagePayload::Handshake(payload) = msg.payload {
            match payload {
                HandshakePayload::Token { token } => {}
                HandshakePayload::Connect { username, password } => {}
                HandshakePayload::Create { username, password } => {}
            }
        } else {
        }

        Ok(())
    }
}

static CONNECTED_BEHAVIOR: ConnectedBehavior = ConnectedBehavior;

struct ConnectedBehavior;

#[async_trait]
impl CommandBehavior for ConnectedBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client, msg: Message) -> Result<()> {
        Ok(())
    }
}
