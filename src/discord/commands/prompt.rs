use crate::constants::*;
use crate::discord::*;
use crate::functions::*;
use crate::tools::get_text;
use async_trait::async_trait;
use std::collections::VecDeque;
use std::error::Error;
use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandType},
        interaction::{application_command::CommandOptionValue, InteractionContextType},
    },
    channel::message::MessageFlags,
    gateway::payload::incoming::InteractionCreate,
};
use twilight_util::builder::command::StringBuilder;
use twilight_util::builder::embed::EmbedBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;
use twilight_util::builder::command::CommandBuilder;

pub struct Prompt;

#[async_trait]
impl SlashCommandHandler for Prompt {
    fn command(&self) -> Command {
        let str_opt = CommandOption::from(StringBuilder::new("text", "Enter the text").build());
        CommandBuilder::new("prompt", "Ask or interact with IA", CommandType::ChatInput)
            .option(str_opt)
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

        let CommandOptionValue::String(question) = &command_data.options[0].value else {
            error("Failed to parse command data");
            return Ok(());
        };

        defer_reply(&ctx, interaction, true).await;

        let response = get_text(question).await?;

        let mut texts = VecDeque::new();
        let mut i = 0;
        let max_length = 3000;

        while i < response.len() {
            let end = usize::min(i + max_length, response.len());
            texts.push_back(response[i..end].to_string());
            i += max_length;
        }

        let embed = EmbedBuilder::new()
            .color(COLORS.green)
            .description(texts.pop_front().unwrap_or_default())
            .build();

        let payload = InteractionResponseDataBuilder::new()
            .flags(MessageFlags::EPHEMERAL)
            .embeds(vec![embed])
            .build();

        update_reply(&ctx, interaction, payload).await;

        while texts.len() > 0 {
            followup_with_embed(
                &ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                COLORS.green,
                &texts.pop_front().unwrap_or_default(),
            )
            .await;
        }

        Ok(())
    }
}
