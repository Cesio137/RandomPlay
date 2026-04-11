use crate::discord::Context;
use crate::functions::error;
use twilight_model::channel::message::component::*;
use twilight_model::channel::message::{Component, MessageFlags};
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_util::builder::message::{
    LabelBuilder, SelectMenuBuilder, TextDisplayBuilder,
};

pub async fn delete_message_modal(ctx: Context, interaction: &Box<InteractionCreate>) {
    let warning = TextDisplayBuilder::new("# Warning").build();
    let warning_text = TextDisplayBuilder::new("Bangboo will automatically filter and remove the guild owner and moderators if any are selected.").build();

    let user_select = LabelBuilder::new(
        "User(s)",
        Component::SelectMenu(
            SelectMenuBuilder::new("users", SelectMenuType::User)
                .max_values(25)
                .min_values(1)
                .placeholder("Select at least one user")
                .build(),
        ),
    )
    .build();

    let limit = LabelBuilder::new(
        "Limit",
        Component::TextInput(TextInput {
            id: None,
            custom_id: "limit".to_string(),
            label: None,
            max_length: Some(4),
            min_length: Some(1),
            placeholder: Some("Numbers from 1 to 1000".into()),
            required: Some(true),
            style: TextInputStyle::Short,
            value: Some("1".into()),
        }),
    )
    .build();

    let modal_response = InteractionResponse {
        kind: InteractionResponseType::Modal,
        data: Some(InteractionResponseData {
            custom_id: Some("modal/moderate/delete-message".into()),
            title: Some("Delete message(s)".into()),
            components: Some(vec![
                Component::TextDisplay(warning),
                Component::TextDisplay(warning_text),
                Component::Label(user_select),
                Component::Label(limit),
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
