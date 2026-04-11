use crate::discord::Context;
use crate::functions::error;
use twilight_model::channel::message::component::*;
use twilight_model::channel::message::{Component, MessageFlags};
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_util::builder::message::{
    LabelBuilder, SelectMenuBuilder, SelectMenuOptionBuilder, TextDisplayBuilder,
};

pub async fn timeout_modal(ctx: Context, interaction: &Box<InteractionCreate>) {
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

    let duration_select = LabelBuilder::new(
        "Duration",
        Component::SelectMenu(
            SelectMenuBuilder::new("duration", SelectMenuType::Text)
                .max_values(1)
                .min_values(1)
                .placeholder("Select duration")
                .option(
                    SelectMenuOptionBuilder::new("60 seconds", "60")
                        .default(true)
                        .build(),
                )
                .option(SelectMenuOptionBuilder::new("05 minutes", "300").build())
                .option(SelectMenuOptionBuilder::new("10 minutes", "600").build())
                .option(SelectMenuOptionBuilder::new("01 hour", "3600").build())
                .option(SelectMenuOptionBuilder::new("01 week", "604800").build())
                .build(),
        ),
    )
    .build();

    let reason_input = LabelBuilder::new(
        "Reason",
        Component::TextInput(TextInput {
            id: None,
            custom_id: "reason".to_string(),
            label: None,
            max_length: Some(500),
            min_length: None,
            placeholder: Some("Visible only in auditlogs channel".into()),
            required: Some(false),
            style: TextInputStyle::Short,
            value: None,
        }),
    )
    .build();

    let modal_response = InteractionResponse {
        kind: InteractionResponseType::Modal,
        data: Some(InteractionResponseData {
            custom_id: Some("modal/moderate/timeout".into()),
            title: Some("Timeout user(s)".into()),

            components: Some(vec![
                Component::TextDisplay(warning),
                Component::TextDisplay(warning_text),
                Component::Label(user_select),
                Component::Label(duration_select),
                Component::Label(reason_input),
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
