use crate::functions::*;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::InteractionResponse;
use twilight_model::http::interaction::InteractionResponseData;
use twilight_model::http::interaction::InteractionResponseType;

pub async fn reply_component(
    http: &Client,
    interaction: &Box<InteractionCreate>,
    payload: InteractionResponseData,
) -> bool {
    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(payload),
    };

    let result = http
        .interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await;

    if let Err(err) = result {
        error(&format!(
            "Error trying to responde command interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}
