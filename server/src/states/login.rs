use crate::prelude::*;
use crate::protocol::*;

use anyhow::Ok;
use bcrypt::{DEFAULT_COST, hash, verify};
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
    log::trace!("[LOGIN.CREATE] start");

    let password_hash = match hash(password, DEFAULT_COST) {
        std::result::Result::Ok(hash) => {
            log::debug!("[LOGIN.CREATE] password hashed: {}", hash);
            hash
        }
        Err(e) => {
            log::error!("[LOGIN.CREATE] hash(): {}", e);
            // send a reply with the code hash error
            return Ok(());
        }
    };

    let id = match sqlx::query_scalar!(
        "INSERT INTO players (username, password_hash) VALUES ($1, $2) RETURNING id",
        username,
        password_hash
    )
    .fetch_one(&*client.sql_pool)
    .await
    {
        std::result::Result::Ok(id) => {
            log::debug!("[LOGIN.CREATE] user created: {}", id);
            id
        }
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            log::debug!("[LOGIN.CREATE] username already used");
            // send a reply with the code already used username
            return Ok(());
        }
        Err(e) => {
            log::error!("[LOGIN.CONNECT] DatabaseError(): {}", e);
            // send a reply with the code database error
            return Ok(());
        }
    };

    client.id = id;
    client.state = ClientState::Homepage;

    match sqlx::query_scalar!(
        "INSERT INTO tokens (player_id, expires_at) VALUES ($1, NOW() + INTERVAL '1 months') RETURNING token",
        client.id
    ).fetch_one(&*client.sql_pool)
    .await {
        std::result::Result::Ok(token) => {
            log::debug!("[LOGIN.CREATE] token generated: {}", token);
            // send a reply with the code login and token generated
        }
        Err(e) => {
            log::error!("[LOGIN.CONNECT] DatabaseError(): {}", e);
            // send a reply with the code login but token error
        }
    };

    log::trace!("[LOGIN.CONNECT] end");
    Ok(())
}

async fn connect_user(client: &mut Client, username: String, password: String) -> Result<()> {
    log::trace!("[LOGIN.CONNECT] start");

    let row = match sqlx::query!(
        "SELECT id, password_hash FROM players WHERE username = $1 LIMIT 1",
        username
    )
    .fetch_one(&*client.sql_pool)
    .await
    {
        std::result::Result::Ok(value) => {
            log::debug!("[LOGIN.CONNECT] username valid");
            value
        }
        Err(sqlx::Error::RowNotFound) => {
            log::debug!("[LOGIN.CONNECT] username invalid");
            // send a reply with the code username invalid
            return Ok(());
        }
        Err(e) => {
            log::error!("[LOGIN.CONNECT] DatabaseError(): {}", e);
            // send a reply with the code database error
            return Ok(());
        }
    };

    match verify(password, &row.password_hash) {
        std::result::Result::Ok(true) => {
            log::debug!("[LOGIN.CONNECT] password valid");
        }
        std::result::Result::Ok(false) => {
            log::debug!("[LOGIN.CONNECT] password invalid");
            // send a reply with the code password invalid
            return Ok(());
        }
        Err(e) => {
            log::error!("[LOGIN.CONNECT] verify(): {}", e);
            // send a reply with the code hash error
            return Ok(());
        }
    }

    client.id = row.id;
    client.state = ClientState::Homepage;

    match sqlx::query_scalar!(
        "INSERT INTO tokens (player_id, expires_at) VALUES ($1, NOW() + INTERVAL '1 months') RETURNING token",
        client.id
    ).fetch_one(&*client.sql_pool)
    .await {
        std::result::Result::Ok(token) => {
            log::debug!("[LOGIN.CONNECT] token generated: {}", token);
            // send a reply with the code login and token generated
        }
        Err(e) => {
            log::error!("[LOGIN.CONNECT] DatabaseError(): {}", e);
            // send a reply with the code login but token error
        }
    };

    log::trace!("[LOGIN.CONNECT] end");
    Ok(())
}

async fn connect_token(client: &mut Client, token: Uuid) -> Result<()> {
    log::trace!("[LOGIN.TOKEN] start");

    let id = match sqlx::query_scalar!(
        "SELECT player_id FROM tokens WHERE token = $1 LIMIT 1",
        token
    )
    .fetch_one(&*client.sql_pool)
    .await
    {
        std::result::Result::Ok(id) => {
            log::debug!("[LOGIN.TOKEN] token valid");
            // send a reply with the code login with token
            id
        }
        Err(sqlx::Error::RowNotFound) => {
            log::debug!("[LOGIN.TOKEN] token invalid");
            // send a reply with the code token invalid
            return Ok(());
        }
        Err(e) => {
            log::error!("[LOGIN.TOKEN] DatabaseError: {}", e);
            // send a reply with the code database error
            return Ok(());
        }
    };

    client.id = id;
    client.state = ClientState::Homepage;

    log::trace!("[LOGIN.TOKEN] end");
    Ok(())
}
