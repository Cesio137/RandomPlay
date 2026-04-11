use std::fmt::format;
use crate::discord::Context;
use crate::functions::error;
use twilight_model::channel::message::component::*;
use twilight_model::channel::message::{Component, MessageFlags};
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::guild::Ban;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_util::builder::message::{LabelBuilder, SelectMenuBuilder, SelectMenuOptionBuilder, TextDisplayBuilder};

pub async fn unban_modal(ctx: Context, interaction: &Box<InteractionCreate>, bans: &[Ban]) {
    let warning = TextDisplayBuilder::new("# Warning").build();
    let warning_text = TextDisplayBuilder::new("Bangboo will automatically filter and remove the guild owner and moderators if any are selected.").build();

    let mut select_menu = SelectMenuBuilder::new("users", SelectMenuType::Text)
        .max_values(bans.len() as u8)
        .min_values(1)
        .placeholder("Select at least one user");

    for ban in bans {
        select_menu = select_menu.option(SelectMenuOptionBuilder::new(&ban.user.name, ban.user.id.to_string())
            .description(ban.reason.as_ref().unwrap_or(&String::new()))
            .build())
    }

    let select_menu = select_menu.build();

    let user_select = LabelBuilder::new(
        "User(s)",
        Component::SelectMenu(
            select_menu
        ),
    )
    .build();

    let modal_response = InteractionResponse {
        kind: InteractionResponseType::Modal,
        data: Some(InteractionResponseData {
            custom_id: Some("modal/moderate/unban".into()),
            title: Some("Unban user(s)".into()),
            components: Some(vec![
                Component::TextDisplay(warning),
                Component::TextDisplay(warning_text),
                Component::Label(user_select),
            ]),
            flags: Some(MessageFlags::IS_COMPONENTS_V2),
            ..Default::default()
        }),
    };

    let res = ctx
        .http
        .interaction(interaction.application_id.clone())
        .create_response(interaction.id.clone(), &interaction.token, &modal_response)
        .await;

    if let Err(err) = res {
        error(&format!("Error trying to respond interaction\n{:?}", err));
    }
}
