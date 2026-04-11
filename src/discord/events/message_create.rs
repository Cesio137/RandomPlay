use crate::{discord::*, tools::mousetrap};
use std::error::Error;
use twilight_gateway::{Event, EventType};
pub struct MessageCreate;

#[async_trait]
impl EventHandler for MessageCreate {
    fn event(&self) -> EventType {
        EventType::MessageCreate
    }

    async fn run(&self, ctx: Context, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        let message = match event {
            Event::MessageCreate(e) => e,
            _ => return Ok(()),
        };

        if let Some(member) = &message.member {
            if let Some(user) = &member.user {
                if user.bot {
                    return Ok(());
                }
            }
        }

        mousetrap(&ctx, &message).await;

        if let Some(callback) = HANDLERS
            .prefix_command_handlers
            .get(message.content.as_str())
        {
            callback.run(ctx, message).await?;
        }

        Ok(())
    }
}
