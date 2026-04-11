use crate::constants::{APPID, COLORS, EMOJIS};
use crate::discord::{Context, MessageComponentHandler};
use crate::functions::{defer_update_reply, error, update_reply, update_with_embed};
use crate::menus::components::{logs_component, status_component};
use crate::tools::{ASCII_REGEX, DISCLOUD};
use async_trait::async_trait;
use std::error::Error;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::util::Timestamp;
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct Status;

#[async_trait]
impl MessageComponentHandler for Status {
    fn custom_id(&self) -> String {
        "discloud/status/refresh".to_string()
    }

    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        defer_update_reply(&ctx, &interaction).await;

        let app = match DISCLOUD.get_app(&APPID).await {
            Ok(apps) => apps.clone(),
            Err(err) => {
                update_with_embed(
                    &ctx,
                    interaction,
                    MessageFlags::EPHEMERAL,
                    COLORS.danger,
                    "Failed to get app",
                )
                .await;
                error(&format!("Failed to get app\n└ {:?}", err));
                return Ok(());
            }
        };

        let status = match app.get_status(&DISCLOUD).await {
            Ok(status) => status,
            Err(err) => {
                update_with_embed(
                    &ctx,
                    interaction,
                    MessageFlags::EPHEMERAL,
                    COLORS.danger,
                    "Failed to fetch app status",
                )
                .await;
                error(&format!("Failed to fetch app status.\n└ {:?}", err));
                return Ok(());
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
        Ok(())
    }
}

pub struct Logs;

#[async_trait]
impl MessageComponentHandler for Logs {
    fn custom_id(&self) -> String {
        "discloud/logs/refresh".to_string()
    }

    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        defer_update_reply(&ctx, &interaction).await;

        let app = match DISCLOUD.get_app(&APPID).await {
            Ok(apps) => apps.clone(),
            Err(err) => {
                update_with_embed(
                    &ctx,
                    interaction,
                    MessageFlags::EPHEMERAL,
                    COLORS.danger,
                    "Failed to get app",
                )
                .await;
                error(&format!("Failed to get app\n└ {:?}", err));
                return Ok(());
            }
        };

        let app_logs = match app.get_logs(&DISCLOUD).await {
            Ok(status) => status,
            Err(err) => {
                update_with_embed(
                    &ctx,
                    interaction,
                    MessageFlags::EPHEMERAL,
                    COLORS.danger,
                    "Failed to fetch app logs",
                )
                .await;
                error(&format!("Failed to fetch app logs\n└ {:?}", err));
                return Ok(());
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
        Ok(())
    }
}
