use crate::client;
use crate::prelude::*;
use crate::protocol::*;

use anyhow::Ok;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum LoginActions {
    Create { username: String, password: String },
    Connect { username: String, password: String },
    Token { token: Uuid },
}

pub struct LoginBehavior;

#[async_trait]
impl CommandBehavior for LoginBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client, msg: Request) -> Result<()> {
        if let RequestTypes::Login(action) = msg.data {
            match action {
                LoginActions::Create { username, password } => {
                    create_user(client, username, password).await?
                }
                LoginActions::Connect { username, password } => {
                    connect_user(client, username, password).await?
                }
                LoginActions::Token { token } => connect_token(client, token).await?,
            }
        } else {
        }

        Ok(())
    }
}

async fn create_user(client: &mut Client, username: String, password: String) -> Result<()> {
    let ret = sqlx::query!(
        "SELECT EXISTS(SELECT 1 FROM players WHERE username = $1) AS exists",
        username
    )
    .fetch_one(&*client.sql_pool)
    .await?;
    if ret.exists.unwrap_or(false) {
        return Ok(());
    } else {
    }
    Ok(())
}

async fn connect_user(client: &mut Client, username: String, password: String) -> Result<()> {
    Ok(())
}

async fn connect_token(client: &mut Client, token: Uuid) -> Result<()> {
    Ok(())
}
