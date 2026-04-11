use twilight_model::{
    channel::message::Embed,
    id::{marker::UserMarker, Id},
    user::User,
};
use twilight_util::builder::embed::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder, ImageSource,
};

use crate::{constants::COLORS, functions::*};

pub fn officer_cui_action_embed(
    user: &User,
    action: &str,
    success_users: &[&Id<UserMarker>],
    failed_users: &[&Id<UserMarker>],
    reason: &str,
) -> Embed {
    let user_name = user.global_name.as_ref().unwrap_or(&user.name);
    let mut embed_author = EmbedAuthorBuilder::new(user_name);
    if let Some(url) = &display_avatar_url(user, 0) {
        embed_author = embed_author.url(url);
    }
    let embed_author = embed_author.build();
    let embed_footer = EmbedFooterBuilder::new(format!("Reason: {}", reason)).build();

    let description = format!("### {} action!", action);

    let mut embed = EmbedBuilder::new()
        .color(COLORS.royal)
        .author(embed_author)
        .title("**Officer Cui's panel**")
        .description(description)
        .footer(embed_footer);

    if let Ok(source) = ImageSource::url(
        "https://raw.githubusercontent.com/BangbooBot/Bot/refs/heads/main/packages/bot/assets/avatar/Officer.png",
    ) {
        embed = embed.thumbnail(source);
    }

    if !success_users.is_empty() {
        let mut field_description = Vec::new();
        for user_id in success_users {
            field_description.push(format!("<@{}>", user_id));
        }
        let field = EmbedFieldBuilder::new("**Sucess**", field_description.join("\n")).build();
        embed = embed.field(field);
    }
    if !failed_users.is_empty() {
        let mut field_description = Vec::new();
        for user_id in failed_users {
            field_description.push(format!("<@{}>", user_id));
        }
        let field = EmbedFieldBuilder::new("**Fail**", field_description.join("\n")).build();
        embed = embed.field(field);
    }

    embed.build()
}
