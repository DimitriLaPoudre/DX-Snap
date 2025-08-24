use crate::protocol::*;

#[derive(Deserialize, Serialize)]
pub enum LoginPayload {
    Token { token: [u8; 32] },
    Connect { username: String, password: String },
    Create { username: String, password: String },
}

pub struct LoginBehavior;

#[async_trait]
impl CommandBehavior for LoginBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client, msg: Message) -> Result<()> {
        if let MessagePayload::Login(payload) = msg.payload {
            match payload {
                LoginPayload::Token { token } => {}
                LoginPayload::Connect { username, password } => {}
                LoginPayload::Create { username, password } => {}
            }
        } else {
        }

        Ok(())
    }
}
