pub use crate::client::Client;
use crate::states::homepage::*;
use crate::states::login::*;

pub use async_trait::async_trait;
pub use serde::{Deserialize, Serialize};
pub use tokio::io::{AsyncReadExt, AsyncWriteExt, Result};

#[derive(Deserialize, Serialize)]
pub struct Message {
    pub seq: u32,
    #[serde(flatten)]
    pub payload: MessagePayload,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MessagePayload {
    Login(LoginPayload),
}

#[derive(Copy, Clone)]
pub enum ClientState {
    Login,
    Homepage,
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
            ClientState::Login => &LOGIN_BEHAVIOR,
            ClientState::Homepage => &HOMEPAGE_BEHAVIOR,
        }
    }
}

#[async_trait]
pub trait CommandBehavior {
    async fn send(&self, client: &mut Client) -> Result<()>;
    async fn received(&self, client: &mut Client, msg: Message) -> Result<()>;
}

static LOGIN_BEHAVIOR: LoginBehavior = LoginBehavior;

static HOMEPAGE_BEHAVIOR: HomepageBehavior = HomepageBehavior;
