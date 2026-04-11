use crate::discord::{Context, SlashCommandHandler};
use crate::functions::{error, get_app_command_data, reply};
use crate::menus::components::fab_component;
use async_trait::async_trait;
use std::error::Error;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::InteractionContextType;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct Fab;

#[async_trait]
impl SlashCommandHandler for Fab {
    fn command(&self) -> Command {
        CommandBuilder::new("fab", "Products on fab marketplace", CommandType::ChatInput)
            .contexts([InteractionContextType::Guild])
            .build()
    }

    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let Some(command_data) = get_app_command_data(interaction) else {
            error("Failed to parse command data");
            return Ok(());
        };

        let component = fab_component();

        let payload = InteractionResponseDataBuilder::new()
            .flags(MessageFlags::EPHEMERAL | MessageFlags::IS_COMPONENTS_V2)
            .components(vec![component])
            .build();

        reply(&ctx, interaction, payload).await;

        Ok(())
    }
}
