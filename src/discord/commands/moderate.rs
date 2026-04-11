use crate::constants::*;
use crate::discord::*;
use crate::functions::*;
use crate::menus::*;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use std::error::Error;
use twilight_model::channel::message::MessageFlags;
use twilight_model::id::marker::RoleMarker;
use twilight_model::id::Id;
use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandType},
        interaction::{application_command::CommandOptionValue, InteractionContextType},
    },
    gateway::payload::incoming::InteractionCreate,
};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::command::StringBuilder;

pub struct Moderate;

#[async_trait]
impl SlashCommandHandler for Moderate {
    fn command(&self) -> Command {
        let user_opt = CommandOption::from(
            StringBuilder::new("action", "Select an action.")
                .required(true)
                .choices([
                    ("delete messages", "delete_messages"),
                    ("timeout", "timeout"),
                    ("kick", "kick"),
                    ("ban", "ban"),
                    ("unban", "unban"),
                ]),
        );
        CommandBuilder::new(
            "moderate",
            "Equality before the law is the cornerstone of justice ⚖.",
            CommandType::ChatInput,
        )
        .option(user_opt)
        .contexts([InteractionContextType::Guild])
        .build()
    }

    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        static KERNEL_ROLE: Lazy<Id<RoleMarker>> =
            Lazy::new(|| Id::<RoleMarker>::new(GUILD.roles.kernel));
        static STF_ROLE: Lazy<Id<RoleMarker>> =
            Lazy::new(|| Id::<RoleMarker>::new(GUILD.roles.stf));

        let Some(command_data) = get_app_command_data(interaction) else {
            error("Failed to get command data");
            return Ok(());
        };

        let Some(member) = interaction.member.as_ref() else {
            error("Error trying to access unknown member");
            return Ok(());
        };

        if !member.roles.contains(&KERNEL_ROLE) && !member.roles.contains(&STF_ROLE) {
            reply_with_embed(
                &ctx,
                interaction,
                MessageFlags::empty(),
                COLORS.danger,
                "You are not a mod or the owner of the guild.",
            )
            .await;
            return Ok(());
        }

        let options = &command_data.options[0];
        let CommandOptionValue::String(action) = &options.value else {
            reply_with_embed(
                &ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                COLORS.danger,
                "Error trying to access unknown option.",
            )
            .await;
            return Ok(());
        };

        match action.as_str() {
            "delete_messages" => {
                delete_message_modal(ctx, interaction).await;
            }
            "timeout" => {
                timeout_modal(ctx, interaction).await;
            }
            "kick" => {
                kick_modal(ctx, interaction).await;
            }
            "ban" => {
                ban_modal(ctx, interaction).await;
            }
            "unban" => {
                /*
                if !defer_reply(&ctx, interaction, true).await {
                    return Ok(());
                }*/

                let Some(guild) = &interaction.guild else {
                    error("Error trying to access unknown guild");
                    return Ok(());
                };
                let Some(guild_id) = guild.id else {
                    error("Error trying to access unknown id from guild");
                    return Ok(());
                };
                let bans = match ctx.http.bans(guild_id).await {
                    Ok(bans) => bans.model().await?,
                    Err(err) => return Err(Box::new(err)),
                };

                if bans.len() <= 0 {
                    reply_with_embed(
                        &ctx,
                        interaction,
                        MessageFlags::EPHEMERAL,
                        COLORS.danger,
                        "There is no banned users on this guild.",
                    )
                    .await;
                    return Ok(());
                }

                unban_modal(ctx, interaction, &bans).await;
            }
            _ => {}
        }

        Ok(())
    }
}
