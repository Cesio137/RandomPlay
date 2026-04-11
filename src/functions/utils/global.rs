use crate::constants::*;
use crate::discord::Context;
use crate::functions::*;
use chrono::Utc;
use skia_safe::{EncodedImageFormat, ISize, Point};
use twilight_model::util::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};
use twilight_gateway::EventType;
use twilight_model::guild::Member;
use twilight_model::http::attachment::Attachment;
use twilight_model::id::marker::ChannelMarker;
use twilight_model::id::Id;
use twilight_model::user::User;
use twilight_util::builder::embed::{EmbedAuthorBuilder, EmbedBuilder, ImageSource};
use twilight_util::snowflake::Snowflake;

pub async fn global_message(
    ctx: &Context,
    channel_id: &Id<ChannelMarker>,
    event: EventType,
    member: Option<&Member>,
    user: &User,
) {
    // Fetch avatar
    let mut user_avatar: Vec<u8> = vec![];

    if let Some(avatar_url) = display_avatar_url(&user, 512) {
        if let Ok(res) = reqwest::get(avatar_url).await {
            if let Ok(bytes) = res.bytes().await {
                user_avatar = bytes.to_vec();
            }
        }
        if user_avatar.is_empty() {
            user_avatar = IMG_DEFAULT_AVATAR.to_vec();
        }
    } else {
        user_avatar = IMG_DEFAULT_AVATAR.to_vec();
    }

    let background = match event {
        EventType::MemberAdd => {
            /*
            let date = match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(duration) => duration.as_millis(),
                Err(_) => 0,
            };
            let join_age = match member.unwrap().joined_at {
                Some(joined_at) => date - (joined_at.as_micros() / 1000) as u128,
                None => 0,
            };
            let account_age = date - user.id.timestamp() as u128;
            const TIME_LIMIT: u128 = 300 * 1000;
            if join_age < TIME_LIMIT || account_age < TIME_LIMIT {
                CARD_NEW
            } else {
                CARD_BACK
            }
            */
            CARD_NEW
        }
        EventType::MemberRemove => CARD_LEFT,
        EventType::BanAdd => CARD_MOD,
        _ => CARD_LEFT,
    };

    let mut data = vec![];
    // Scope to fix safe issue with skia
    {
        let mut surface = match skia_safe::surfaces::raster_n32_premul(ISize {
            width: 2800,
            height: 560,
        }) {
            Some(surface) => surface,
            None => {
                error("Failed to create canvas surface.");
                return;
            }
        };
        let canvas = surface.canvas();

        let background_image = match load_image_from_bytes(background) {
            Some(image) => image,
            None => {
                error("Failed to load background image.");
                return;
            }
        };
        canvas.draw_image(&background_image, Point { x: 0.0, y: 0.0 }, None);
        canvas.save();

        // Avatar
        let cdn_avatar = match load_image_from_bytes(&user_avatar) {
            Some(image) => image,
            None => {
                error("Failed to decode user avatar image.");
                return;
            }
        };
        let avatar = match resize_image(cdn_avatar, 400, 400) {
            Some(image) => image,
            None => {
                error("Failed to resize user avatar image.");
                return;
            }
        };
        draw_circle(canvas, avatar, 204.0, 360.0, 200.0);

        if !draw_text_with_font(canvas, &user.name, FONT_FREDOKA, 200.0, 530.0, 140.0) {
            error("Failed to resize user avatar image.");
            return;
        }

        let nickname = match user.global_name.as_ref() {
            Some(nickname) => nickname,
            None => "Undefined",
        };
        if !draw_text_with_font(
            canvas,
            &format!("@{}", nickname),
            FONT_ROBOTO,
            96.0,
            530.0,
            380.0,
        ) {
            error("Failed to resize user avatar image.");
            return;
        }

        let image = surface.image_snapshot();
        let encoded_data = match image.encode(None, EncodedImageFormat::PNG, Some(100)) {
            Some(data) => data,
            None => {
                error("Failed to encode card image.");
                return;
            }
        };

        data = encoded_data.to_vec();
    }

    let mut utc = String::new();
    if event == EventType::MemberAdd {
        let joined_at = match member.unwrap().joined_at {
            Some(timestamp) => timestamp.as_secs(),
            None => 0,
        };
        utc.push_str(&format!("<t:{}:F>", joined_at));
    } else {
        let timeout_until = Utc::now();
        match Timestamp::from_micros(timeout_until.timestamp_micros()) {
            Ok(time) => {
                utc = format!("<t:{}:F>", time.as_secs());
            },
            Err(err) => {
                error(&format!(
                    "Error trying to parse DataTime to Timestamp\n└ {:?}",
                    err
                ));
            }
        };
    }
    let attachment = Attachment::from_bytes("Card.png".to_string(), data, 0);

    let res = ctx
        .http
        .create_message(channel_id.clone())
        .attachments(&[attachment])
        .content(&utc)
        .await;

    if let Err(err) = res {
        error(&format!(
            "Error trying to send card to system channel\nʟ {:?}",
            err
        ));
    }
}

pub async fn global_boost(ctx: &Context, user: &User, channel_id: &Id<ChannelMarker>) {
    let avatar_url = display_avatar_url(user, 256).unwrap_or(String::new());
    let username = user.global_name.clone().unwrap_or(user.name.clone());
    let description = format!(
        "**<a:boost:{}> <@${}> became a <@&${}>**\n\n🚀 Thanks for boosting the server!",
        &EMOJIS.animated.boost, user.id, &GUILD.roles.boosters
    );

    let mut embed = EmbedBuilder::new()
        .color(COLORS.nitro)
        .description(description);

    if let Ok(author) = ImageSource::url(&avatar_url) {
        embed = embed.author(
            EmbedAuthorBuilder::new(username.as_str())
                .icon_url(author.clone())
                .build(),
        );
        embed = embed.thumbnail(author);
    }

    let embed = embed.build();

    if let Err(err) = ctx
        .http
        .create_message(channel_id.clone())
        .embeds(&[embed])
        .await
    {
        error(&format!("Failed to send message\n└ {:?}", err));
    }
}
