use crate::protocol::*;

pub struct HomepageBehavior;

#[async_trait]
impl CommandBehavior for HomepageBehavior {
    async fn send(&self, client: &mut Client) -> Result<()> {
        Ok(())
    }

    async fn received(&self, client: &mut Client, msg: Message) -> Result<()> {
        Ok(())
    }
}
