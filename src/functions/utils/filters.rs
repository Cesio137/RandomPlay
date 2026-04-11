use crate::{constants::GUILD, discord::Context};
use once_cell::sync::Lazy;
use std::error::Error;
use twilight_cache_inmemory::CacheableMember;
use twilight_model::id::marker::GuildMarker;
use twilight_model::id::{
    marker::{RoleMarker, UserMarker},
    Id,
};

pub fn filter_messages_by_user_id(ctx: &Context, user_ids: &[Id<UserMarker>]) {}

pub async fn filter_users_from_mod_action(
    ctx: &Context,
    guild_id: &Id<GuildMarker>,
    user_ids: &[Id<UserMarker>],
) -> Result<Vec<Id<UserMarker>>, Box<dyn Error + Send + Sync>> {
    static KERNEL_ROLE: Lazy<Id<RoleMarker>> =
        Lazy::new(|| Id::<RoleMarker>::new(GUILD.roles.kernel));
    static STF_ROLE: Lazy<Id<RoleMarker>> = Lazy::new(|| Id::<RoleMarker>::new(GUILD.roles.stf));

    let mut filtered_users = Vec::with_capacity(user_ids.len());
    for id in user_ids {
        let roles = match ctx.cache.member(guild_id.clone(), id.clone()) {
            Some(member) => member.roles().to_vec(),
            None => {
                let member = ctx
                    .http
                    .guild_member(guild_id.clone(), id.clone())
                    .await?
                    .model()
                    .await?;
                member.roles.clone()
            }
        };
        if !roles.contains(&KERNEL_ROLE) && !roles.contains(&STF_ROLE) {
            filtered_users.push(id.clone());
        }
    }

    Ok(filtered_users)
}
