use std::mem;

use twilight_http::request::AuditLogReason;
use twilight_model::channel::Message;
use chrono::Utc;
use twilight_util::builder::embed::EmbedBuilder;
use crate::constants::*;
use crate::discord::*;
use crate::functions::*;

pub async fn mousetrap(ctx: &Context, message: &Message) {
    if message.channel_id.get() != GUILD.channels.mousetrap {
        return;
    }

    let guild_id = match message.guild_id.as_ref() {
        Some(guild_id) => guild_id,
        None => {
            return;
        }
    };

    let id = message.author.id;
    let reason = "Captured by mousetrap.";

    let now = Utc::now().timestamp();
    let mut should_unban = false;

    if let Some(member) = message.member.as_ref() {
        if let Some(joined_at) = member.joined_at {
            let joined = joined_at.as_secs();
            if now - joined > (60 * 60 * 24 * 7) {
                should_unban = true;
            }
        }
    }

    let username = message.author.name.as_str();

    if should_unban {
        if let Ok(dm_channel) = ctx.http.create_private_channel(id).await {
            if let Ok(channel) = dm_channel.model().await {
                let dm_content = "Hello! Bangboo has identified that your account sent a series of suspicious messages on the server's channels. As you are a long-time member of the server, your ban has been automatically removed since your account was likely hacked. I recommend that you review your account security!";
                let embed = EmbedBuilder::new()
                    .description(dm_content)
                    .color(COLORS.warning)
                    .build();
                if let Err(err) = ctx.http.create_message(channel.id)
                    .embeds(&[embed])
                    .await {
                        error(&format!("Failed to send DM to {} after being captured by mousetrap\n└ {:?}", username, err));
                    }
            }
        }
    }

    if let Err(err) = ctx.http
        .create_ban(guild_id.clone(), id)
        .delete_message_seconds(604_800)
        .reason(reason)
        .await {
        error(&format!("Failed to ban user\n└ {:?}", err));
        return;
    }

    if should_unban {
        if let Err(err) = ctx.http.delete_ban(guild_id.clone(), id).await {
            error(&format!("Failed to unban {} after being captured by mousetrap\n└ {:?}", username, err));
        }
    }
}
