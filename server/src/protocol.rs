pub use crate::client::Client;

use crate::prelude::*;
use crate::states::homepage::*;
use crate::states::login::*;

pub use async_trait::async_trait;
pub use futures_util::sink::SinkExt;
pub use futures_util::stream::StreamExt;
pub use serde::{Deserialize, Serialize};
pub use tokio_tungstenite::tungstenite::protocol::Message;

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
        match client
            .reader
            .next()
            .await
            .ok_or(anyhow!("connection lost"))?
        {
            Ok(frame) => match frame {
                Message::Text(txt) => self.behavior().received(client, txt.to_string()).await,
                Message::Close(cf) => {
                    client.writer.send(Message::Close(cf)).await?;
                    Err(anyhow!("connection closed"))
                }
                _ => Ok(()),
            },
            Err(e) => Err(anyhow!(e)),
        }
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
    async fn received(&self, client: &mut Client, txt: String) -> Result<()>;
}

static LOGIN_BEHAVIOR: LoginBehavior = LoginBehavior;

static HOMEPAGE_BEHAVIOR: HomepageBehavior = HomepageBehavior;
