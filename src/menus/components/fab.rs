use crate::constants::{COLORS, EMOJIS, FAB};
use twilight_model::channel::message::component::{ButtonStyle, SeparatorSpacingSize};
use twilight_model::channel::message::{Component, EmojiReactionType};
use twilight_model::id::marker::EmojiMarker;
use twilight_model::id::Id;
use twilight_util::builder::message::{
    ActionRowBuilder, ButtonBuilder, ContainerBuilder, SeparatorBuilder, TextDisplayBuilder,
};

pub fn fab_component() -> Component {
    let text_display = TextDisplayBuilder::new("### CHECK OUT MY FAB PRODUCTS").build();
    let separator = SeparatorBuilder::new()
        .spacing(SeparatorSpacingSize::Small)
        .build();
    let eus_row = ActionRowBuilder::new()
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Engine User Settings")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_f),
                    name: None,
                    animated: false,
                })
                .url(&FAB.engine_user_setings.product_url)
                .build(),
        ))
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Documentation")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_d),
                    name: None,
                    animated: false,
                })
                .url(&FAB.engine_user_setings.doc_url)
                .build(),
        ))
        .build();
    let eus_desc = TextDisplayBuilder::new(&FAB.engine_user_setings.product_desc).build();

    let ip_row = ActionRowBuilder::new()
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Internet Protocol")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_f),
                    name: None,
                    animated: false,
                })
                .url(&FAB.internet_protocol.product_url)
                .build(),
        ))
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Link)
                .label("Documentation")
                .emoji(EmojiReactionType::Custom {
                    id: Id::<EmojiMarker>::new(EMOJIS.emojis_static.icons_d),
                    name: None,
                    animated: false,
                })
                .url(&FAB.internet_protocol.doc_url)
                .build(),
        ))
        .build();
    let ip_desc = TextDisplayBuilder::new(&FAB.internet_protocol.product_desc).build();

    Component::Container(
        ContainerBuilder::new()
            .accent_color(Some(COLORS.green))
            .component(Component::TextDisplay(text_display))
            .component(Component::Separator(separator.clone()))
            .component(Component::ActionRow(eus_row))
            .component(Component::TextDisplay(eus_desc))
            .component(Component::Separator(separator))
            .component(Component::ActionRow(ip_row))
            .component(Component::TextDisplay(ip_desc))
            .build(),
    )
}
