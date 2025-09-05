use crate::prelude::*;
use crate::protocol::*;

pub struct HomepageBehavior;

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type")]
pub enum RequestHomepage {
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

    async fn received(&self, client: &mut Client, txt: String) -> Result<()> {
        let msg: RequestHomepage = match serde_json::from_str(&txt.to_string()) {
            Ok(msg) => {
                log::debug!("[HOMEPAGE] request received: {msg:?}");
                msg
            }
            Err(e) => {
                log::error!("[HOMEPAGE] serde_json(): {e}");
                return Ok(());
            }
        };
        match msg {
            RequestHomepage::Play {} => {}
            RequestHomepage::Collection {} => {}
            RequestHomepage::Settings {} => {}
            RequestHomepage::Deck {} => {}
        }

        Ok(())
    }
}
