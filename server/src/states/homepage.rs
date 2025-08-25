use crate::prelude::*;
use crate::protocol::*;

pub struct HomepageBehavior;

#[derive(Deserialize, Serialize)]
pub enum HomepagePayload {
    Play {},
    Collection {},
    Settings {},
    Deck {},
}

#[async_trait]
impl CommandBehavior for HomepageBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client, msg: Message) -> Result<()> {
        if let MessagePayload::Homepage(payload) = msg.payload {
        } else {
        }

        Ok(())
    }
}
