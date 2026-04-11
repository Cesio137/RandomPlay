use crate::{
    discord::*,
    functions::{error, global_message},
};
use std::error::Error;
use twilight_gateway::{Event, EventType};

pub struct BanAdd;

#[async_trait]
impl EventHandler for BanAdd {
    fn event(&self) -> EventType {
        EventType::BanAdd
    }

    async fn run(&self, ctx: Context, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        let ban_add = match event {
            Event::BanAdd(e) => e,
            _ => return Ok(()),
        };

        if ban_add.user.bot {
            return Ok(());
        }

        let guild_id = &ban_add.guild_id;

        let mut system_channel_id = None;

        if let Some(cached_guild) = ctx.cache.guild(guild_id.clone()) {
            if let Some(sys_channel_id) = cached_guild.system_channel_id() {
                system_channel_id = Some(sys_channel_id);
            }
        }

        if system_channel_id.is_none() {
            if let Ok(guild) = ctx.http.guild(guild_id.clone()).await?.model().await {
                if let Some(sys_channel_id) = guild.system_channel_id {
                    system_channel_id = Some(sys_channel_id);
                }
            }
        }

        if system_channel_id.is_none() {
            error("System channel not set for guild.");
            return Ok(());
        }

        let system_channel_id = system_channel_id.unwrap();

        let user = &ban_add.user;

        global_message(&ctx, &system_channel_id, EventType::BanAdd, None, user).await;

        Ok(())
    }
}
