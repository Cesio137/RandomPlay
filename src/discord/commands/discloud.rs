use crate::constants::*;
use crate::discord::*;
use crate::functions::*;
use crate::menus::components::{logs_component, status_component};
use crate::tools::{ASCII_REGEX, DISCLOUD};
use async_trait::async_trait;
use std::error::Error;
use twilight_model::util::Timestamp;
use twilight_model::{
    application::{
        command::{Command, CommandOption, CommandType},
        interaction::{application_command::CommandOptionValue, InteractionContextType},
    },
    channel::message::MessageFlags,
    gateway::payload::incoming::InteractionCreate,
};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::command::StringBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct Discloud;

#[async_trait]
impl SlashCommandHandler for Discloud {
    fn command(&self) -> Command {
        let str_opt = CommandOption::from(
            StringBuilder::new("fetch", "Select info to fetch")
                .choices(vec![("status", "status"), ("logs", "logs")])
                .build(),
        );
        CommandBuilder::new(
            "discloud",
            "Show the bot status or logs from host",
            CommandType::ChatInput,
        )
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

        let Some(guild_id) = &interaction.guild_id else {
            error("Guild id not found.");
            return Ok(());
        };

        defer_reply(&ctx, interaction, true).await;

        if let CommandOptionValue::String(str_value) = &command_data.options[0].value {
            match str_value.as_str() {
                "status" => status(&ctx, interaction).await,
                "logs" => logs(&ctx, interaction).await,
                _ => {}
            }
        }

        Ok(())
    }
}

async fn status(ctx: &Context, interaction: &Box<InteractionCreate>) {
    let app = match DISCLOUD.get_app(&APPID).await {
        Ok(apps) => apps.clone(),
        Err(err) => {
            update_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                COLORS.danger,
                "Failed to get app",
            )
            .await;
            error(&format!("Failed to get app\n└ {:?}", err));
            return;
        }
    };

    let status = match app.get_status(&DISCLOUD).await {
        Ok(status) => status,
        Err(err) => {
            update_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                COLORS.danger,
                "Failed to fetch app status",
            )
            .await;
            error(&format!("Failed to fetch app status.\n└ {:?}", err));
            return;
        }
    };
    let mut infos = Vec::new();
    infos.push(format!(
        "<:id:{}>`Nome(ID):` **{}({})**",
        EMOJIS.emojis_static.id, app.name, app.id
    ));
    infos.push(format!(
        "<:cpu:{}>`CPU:` **{}**",
        EMOJIS.emojis_static.cpu, status.cpu
    ));
    infos.push(format!(
        "<:ram:{}>`RAM:` **{}**",
        EMOJIS.emojis_static.ram, status.memory
    ));
    infos.push(format!(
        "<:wifi:{}>`Network:`  `⬆`**{}** `⬇`**{}**",
        EMOJIS.emojis_static.wifi, status.net_io.up, status.net_io.down
    ));
    if let Ok(timestamp) = Timestamp::parse(&status.started_at) {
        infos.push(format!(
            "<:refresh:{}>`Latest restart:` **<t:{}:R>**",
            EMOJIS.emojis_static.refresh,
            timestamp.as_secs()
        ));
    } else {
        infos.push(format!(
            "<:refresh:{}>`Latest restart:` **{}**",
            EMOJIS.emojis_static.refresh, &status.last_restart
        ));
    }

    let component = status_component(infos);
    let payload = InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL | MessageFlags::IS_COMPONENTS_V2)
        .components(vec![component])
        .build();
    update_reply(&ctx, interaction, payload).await;
}

async fn logs(ctx: &Context, interaction: &Box<InteractionCreate>) {
    let app = match DISCLOUD.get_app(&APPID).await {
        Ok(apps) => apps.clone(),
        Err(err) => {
            update_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                COLORS.danger,
                "Failed to get app",
            )
            .await;
            error(&format!("Failed to get app\n└ {:?}", err));
            return;
        }
    };

    let app_logs = match app.get_logs(&DISCLOUD).await {
        Ok(status) => status,
        Err(err) => {
            update_with_embed(
                ctx,
                interaction,
                MessageFlags::EPHEMERAL,
                COLORS.danger,
                "Failed to fetch app logs",
            )
            .await;
            error(&format!("Failed to fetch app logs\n└ {:?}", err));
            return;
        }
    };

    let mut logs = app_logs.terminal.small.unwrap_or("".to_string());
    logs = ASCII_REGEX.replace_all(&logs, "").to_string();

    let component = logs_component(&logs);
    let payload = InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL | MessageFlags::IS_COMPONENTS_V2)
        .components(vec![component])
        .build();
    update_reply(&ctx, interaction, payload).await;
}
