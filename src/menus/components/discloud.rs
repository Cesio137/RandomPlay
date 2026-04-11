use crate::constants::*;
use twilight_model::channel::message::component::{ButtonStyle, SeparatorSpacingSize};
use twilight_model::channel::message::Component;
use twilight_util::builder::message::{
    ActionRowBuilder, ButtonBuilder, ContainerBuilder, SeparatorBuilder, TextDisplayBuilder,
};

pub fn status_component(infos: Vec<String>) -> Component {
    let text_display = TextDisplayBuilder::new("### BANGBOO'S STATUS").build();
    let separator = SeparatorBuilder::new()
        .spacing(SeparatorSpacingSize::Large)
        .build();
    let info_display = TextDisplayBuilder::new(infos.join("\n")).build();
    let refresh_row = ActionRowBuilder::new()
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Success)
                .label("Refresh")
                .custom_id("discloud/status/refresh")
                .build(),
        ))
        .build();

    Component::Container(
        ContainerBuilder::new()
            .accent_color(Some(COLORS.green))
            .component(Component::TextDisplay(text_display))
            .component(Component::Separator(separator.clone()))
            .component(Component::TextDisplay(info_display))
            .component(Component::Separator(separator))
            .component(Component::ActionRow(refresh_row))
            .build(),
    )
}

pub fn logs_component(logs: &str) -> Component {
    let text_display = TextDisplayBuilder::new("### BANGBOO'S LOGS").build();
    let separator = SeparatorBuilder::new()
        .spacing(SeparatorSpacingSize::Large)
        .build();
    let info_display = TextDisplayBuilder::new(format!("```bash\n{logs}\n```")).build();
    let refresh_row = ActionRowBuilder::new()
        .component(Component::Button(
            ButtonBuilder::new(ButtonStyle::Success)
                .label("Refresh")
                .custom_id("discloud/logs/refresh")
                .build(),
        ))
        .build();

    Component::Container(
        ContainerBuilder::new()
            .accent_color(Some(COLORS.green))
            .component(Component::TextDisplay(text_display))
            .component(Component::Separator(separator.clone()))
            .component(Component::TextDisplay(info_display))
            .component(Component::Separator(separator))
            .component(Component::ActionRow(refresh_row))
            .build(),
    )
}
