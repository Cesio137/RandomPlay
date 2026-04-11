use crate::constants::GUILD;
use crate::discord::*;
use crate::functions::global_boost;
use std::error::Error;
use twilight_gateway::{Event, EventType};
use twilight_model::id::marker::{ChannelMarker, RoleMarker};
use twilight_model::id::Id;

pub struct MemberUpdate;

#[async_trait]
impl EventHandler for MemberUpdate {
    fn event(&self) -> EventType {
        EventType::MemberUpdate
    }

    async fn run(&self, ctx: Context, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        let member_update = match event {
            Event::MemberUpdate(e) => e,
            _ => return Ok(()),
        };

        let guild_id = &member_update.guild_id;
        let user_id = &member_update.user.id;
        let channel_id = Id::<ChannelMarker>::new(GUILD.channels.announcement);

        if member_update.user.bot {
            return Ok(());
        }

        let booster_role = Id::<RoleMarker>::new(GUILD.roles.boosters);
        if member_update.premium_since.is_some() {
            ctx.http
                .add_guild_member_role(guild_id.clone(), user_id.clone(), booster_role)
                .await?;
            global_boost(&ctx, &member_update.user, &channel_id).await;
        } else {
            ctx.http
                .remove_guild_member_role(guild_id.clone(), user_id.clone(), booster_role)
                .await?;
        }

        Ok(())
    }
}
