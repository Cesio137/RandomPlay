use crate::constants::{COLORS, EMOJIS};
use twilight_model::channel::message::component::{ButtonStyle, SeparatorSpacingSize};
use twilight_model::channel::message::{Component, EmojiReactionType};
use twilight_model::id::marker::EmojiMarker;
use twilight_model::id::Id;
use twilight_util::builder::message::{
    ActionRowBuilder, ButtonBuilder, ContainerBuilder, SeparatorBuilder, TextDisplayBuilder,
};

pub fn social_component() -> Component {
    let text_display = TextDisplayBuilder::new("## FOLLOW ME ON SOCIAL MEDIA").build();
    let separator = SeparatorBuilder::new()
        .spacing(SeparatorSpacingSize::Small)
        .build();
    let social_row = ActionRowBuilder::new()
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Portifolio")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_p),
                    name: None,
                    animated: false,
                })
                .url("https://nathan-miguel.vercel.app/")
                .build(),
        ))
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Youtube")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_youtube),
                    name: None,
                    animated: false,
                })
                .url("https://www.youtube.com/@NathanMiguel1")
                .build(),
        ))
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Instagram")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_instagram),
                    name: None,
                    animated: false,
                })
                .url("https://www.instagram.com/nathan_cmiguel/")
                .build(),
        ))
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Github")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_github),
                    name: None,
                    animated: false,
                })
                .url("https://github.com/nathancmiguel")
                .build(),
        ))
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("X/Twitter")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_x),
                    name: None,
                    animated: false,
                })
                .url("https://x.com/nathancmig")
                .build(),
        ))
        .build();

    let fab_separator = SeparatorBuilder::new()
        .spacing(SeparatorSpacingSize::Large)
        .divider(true)
        .build();

    let fab_text_display = TextDisplayBuilder::new("## VISIT MY FAB STORE").build();

    let fab_row = ActionRowBuilder::new()
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Fab")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_f),
                    name: None,
                    animated: false,
                })
                .url("https://www.fab.com/sellers/Nathan%20Miguel")
                .build(),
        ))
        .build();

    Component::Container(
        ContainerBuilder::new()
            .accent_color(Some(COLORS.green))
            .component(Component::TextDisplay(text_display))
            .component(Component::Separator(separator))
            .component(Component::ActionRow(social_row))
            .component(Component::Separator(fab_separator))
            .component(Component::TextDisplay(fab_text_display))
            .component(Component::ActionRow(fab_row))
            .build(),
    )
}
