use crate::prelude::*;
use crate::protocol::*;

pub struct HomepageBehavior;

#[derive(Deserialize, Serialize)]
#[serde(tag = "action")]
pub enum HomepageActions {
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

    async fn received(&self, client: &mut Client, msg: Request) -> Result<()> {
        if let RequestTypes::Homepage(action) = msg.data {
        } else {
        }

        Ok(())
    }
}
