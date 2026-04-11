use std::collections::HashMap;

use crate::discord::Context;
use crate::functions::*;
use twilight_model::application::interaction::application_command::CommandData;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::application::interaction::modal::{
    ModalInteractionComponent, ModalInteractionData,
};
use twilight_model::application::interaction::InteractionData;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::http::interaction::{
    InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_util::builder::embed::EmbedBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

pub fn get_app_command_data(interaction: &Box<InteractionCreate>) -> Option<&Box<CommandData>> {
    if let Some(data) = &interaction.data {
        match data {
            InteractionData::ApplicationCommand(command) => {
                return Some(command);
            }
            _ => {}
        }
    }
    None
}

pub fn get_modal_data(interaction: &Box<InteractionCreate>) -> Option<&Box<ModalInteractionData>> {
    if let Some(data) = &interaction.data {
        match data {
            InteractionData::ModalSubmit(modal) => {
                return Some(modal);
            }
            _ => {}
        }
    }
    None
}

pub fn get_message_component_data(
    interaction: &Box<InteractionCreate>,
) -> Option<&Box<MessageComponentInteractionData>> {
    if let Some(data) = &interaction.data {
        match data {
            InteractionData::MessageComponent(message) => {
                return Some(message);
            }
            _ => {}
        }
    }
    None
}

pub fn modal_labels_to_hash(
    components: &[ModalInteractionComponent],
) -> HashMap<String, ModalInteractionComponent> {
    let mut hashmap: HashMap<String, ModalInteractionComponent> = HashMap::new();
    for component in components {
        let mut id = "";
        let mut modal_component: ModalInteractionComponent = ModalInteractionComponent::Unknown(0);
        if let ModalInteractionComponent::Label(label) = component {
            match label.component.as_ref() {
                ModalInteractionComponent::UserSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::StringSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::ChannelSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::MentionableSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::RoleSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::TextInput(input_text) => {
                    id = input_text.custom_id.as_str();
                }
                _ => {
                    continue;
                }
            }
            modal_component = label.component.as_ref().clone();
        } else {
            match component {
                ModalInteractionComponent::UserSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::StringSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::ChannelSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::MentionableSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::RoleSelect(select) => {
                    id = select.custom_id.as_str();
                }
                ModalInteractionComponent::TextInput(input_text) => {
                    id = input_text.custom_id.as_str();
                }
                _ => {
                    continue;
                }
            }
            modal_component = component.clone();
        }
        hashmap.insert(id.to_string(), modal_component);
    }
    hashmap
}

pub async fn reply(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    payload: InteractionResponseData,
) -> bool {
    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(payload),
    };

    let result = ctx
        .http
        .interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await;

    if let Err(err) = result {
        let cmd_name = match get_app_command_data(interaction) {
            Some(cmd) => cmd.name.as_str(),
            None => "",
        };
        error(&format!(
            "Error trying to responde command interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}

pub async fn defer_reply(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    ephemeral: bool,
) -> bool {
    let res_data = if ephemeral {
        Some(
            InteractionResponseDataBuilder::new()
                .flags(MessageFlags::EPHEMERAL)
                .build(),
        )
    } else {
        None
    };
    let response = InteractionResponse {
        kind: InteractionResponseType::DeferredChannelMessageWithSource,
        data: res_data,
    };

    let result = ctx
        .http
        .interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await;

    if let Err(err) = result {
        error(&format!(
            "Error trying to defer response to interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}

pub async fn update_reply(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    payload: InteractionResponseData,
) -> bool {
    let interaction_api = ctx.http.interaction(interaction.application_id);
    let mut response = interaction_api.update_response(&interaction.token);

    if let Some(data) = &payload.allowed_mentions {
        response = response.allowed_mentions(Some(data));
    }

    if let Some(data) = &payload.attachments {
        response = response.attachments(data);
    }

    if let Some(data) = &payload.components {
        response = response.components(Some(data));
    }

    if let Some(data) = &payload.content {
        response = response.content(Some(data));
    }

    if let Some(data) = &payload.embeds {
        response = response.embeds(Some(data));
    }

    if let Some(data) = &payload.flags {
        response = response.flags(data.clone());
    }

    let result = response.await;

    if let Err(err) = result {
        error(&format!(
            "Error trying to update command interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}

pub async fn defer_update_reply(ctx: &Context, interaction: &Box<InteractionCreate>) -> bool {
    let response = InteractionResponse {
        kind: InteractionResponseType::DeferredUpdateMessage,
        data: None,
    };

    let result = ctx
        .http
        .interaction(interaction.application_id)
        .create_response(interaction.id, &interaction.token, &response)
        .await;

    if let Err(err) = result {
        error(&format!(
            "Error trying to defer response to interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}

pub async fn followup_reply(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    payload: InteractionResponseData,
) -> bool {
    let interaction_api = ctx.http.interaction(interaction.application_id);
    let mut response = interaction_api.create_followup(&interaction.token);

    if let Some(data) = &payload.allowed_mentions {
        response = response.allowed_mentions(Some(data));
    }

    if let Some(data) = &payload.attachments {
        response = response.attachments(data);
    }

    if let Some(data) = &payload.components {
        response = response.components(data);
    }

    if let Some(data) = &payload.content {
        response = response.content(data);
    }

    if let Some(data) = &payload.embeds {
        response = response.embeds(data);
    }

    if let Some(data) = &payload.flags {
        response = response.flags(data.clone());
    }

    let result = response.await;

    if let Err(err) = result {
        let cmd_name = match get_app_command_data(interaction) {
            Some(cmd) => cmd.name.as_str(),
            None => "",
        };
        error(&format!(
            "Error trying to responde command interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}

pub async fn reply_with_embed(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    flags: MessageFlags,
    color: u32,
    content: &str,
) -> bool {
    let embed = EmbedBuilder::new()
        .color(color)
        .description(content)
        .build();

    let res_message = InteractionResponseDataBuilder::new()
        .embeds(vec![embed])
        .flags(flags)
        .build();

    let response = InteractionResponse {
        kind: InteractionResponseType::ChannelMessageWithSource,
        data: Some(res_message),
    };

    let result = ctx
        .http
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

pub async fn update_with_embed(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    flags: MessageFlags,
    color: u32,
    content: &str,
) -> bool {
    let embed = EmbedBuilder::new()
        .color(color)
        .description(content)
        .build();

    let result = ctx
        .http
        .interaction(interaction.application_id)
        .update_response(&interaction.token)
        .embeds(Some(&[embed]))
        .flags(flags)
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

pub async fn followup_with_embed(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    flags: MessageFlags,
    color: u32,
    content: &str,
) -> bool {
    let embed = EmbedBuilder::new()
        .color(color)
        .description(content)
        .build();

    let result = ctx
        .http
        .interaction(interaction.application_id)
        .create_followup(&interaction.token)
        .embeds(&[embed])
        .flags(flags)
        .await;

    if let Err(err) = result {
        let cmd_name = match get_app_command_data(interaction) {
            Some(cmd) => cmd.name.as_str(),
            None => "",
        };
        error(&format!(
            "Error trying to responde command interaction!\n└ {:?}",
            err
        ));
        return false;
    }

    true
}
