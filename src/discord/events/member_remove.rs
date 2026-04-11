use crate::{
    discord::*,
    functions::{error, global_message},
};
use std::error::Error;
use twilight_gateway::{Event, EventType};

pub struct MemberRemove;

#[async_trait]
impl EventHandler for MemberRemove {
    fn event(&self) -> EventType {
        EventType::MemberRemove
    }

    async fn run(&self, ctx: Context, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        let member_remove = match event {
            Event::MemberRemove(e) => e,
            _ => return Ok(()),
        };

        if member_remove.user.bot {
            return Ok(());
        }

        let guild_id = &member_remove.guild_id;

        if ctx
            .http
            .ban(guild_id.clone(), member_remove.user.id.clone())
            .await.is_ok() {
                return Ok(());
        }
            

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

        let user = &member_remove.user;

        global_message(
            &ctx,
            &system_channel_id,
            EventType::MemberRemove,
            None,
            user,
        )
        .await;

        Ok(())
    }
}
