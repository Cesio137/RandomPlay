use crate::discord::{Context, ModalHandler};
use crate::functions::{
    defer_reply, error, get_modal_data, modal_labels_to_hash, officer_cui_action_embed,
    update_reply,
};
use async_trait::async_trait;
use std::error::Error;
use twilight_model::application::interaction::modal::ModalInteractionComponent;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct DeleteMessage;

#[async_trait]
impl ModalHandler for DeleteMessage {
    fn custom_id(&self) -> String {
        "modal/moderate/delete-message".into()
    }

    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let modal_data = get_modal_data(interaction).unwrap();

        defer_reply(&ctx, interaction, true).await;

        let modal_components = modal_labels_to_hash(&modal_data.components);

        let users = if let Some(component) = modal_components.get("users") {
            let ModalInteractionComponent::UserSelect(menu) = component else {
                error("Failed to parse user field.");
                return Ok(());
            };
            menu.values.as_slice()
        } else {
            error("Failed to parse user field.");
            return Ok(());
        };

        let limit = if let Some(component) = modal_components.get("limit") {
            match &component {
                ModalInteractionComponent::TextInput(modal_interaction_text_input) => {
                    modal_interaction_text_input
                        .value
                        .as_str()
                        .parse::<u16>()
                        .unwrap_or(0)
                }
                _ => 0 as u16,
            }
        } else {
            error("Failed to parse reason field.");
            return Ok(());
        };

        delete_message_action(&ctx, interaction, users, limit).await;

        Ok(())
    }
}

pub async fn delete_message_action(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    ids: &[Id<UserMarker>],
    limit: u16,
) {
    let Some(guild_id) = interaction.guild_id.as_ref() else {
        error("Error trying to access unknow guild.");
        return;
    };

    let Some(channel) = interaction.channel.as_ref() else {
        error("Error trying to access unknown channel.");
        return;
    };

    let user = match interaction.member.as_ref() {
        Some(member) => {
            if let Some(user) = member.user.as_ref() {
                user
            } else {
                error("Error trying to access unknow user.");
                return;
            }
        }
        None => {
            error("Error trying to access unknow member.");
            return;
        }
    };

    let mut success = Vec::new();
    let mut failed = Vec::new();

    let messages = match ctx.http.channel_messages(channel.id).limit(limit).await {
        Ok(res) => {
            let mut body_msg = match res.model().await {
                Ok(model) => model,
                Err(err) => {
                    error(&format!("Failed to deserialize messages\n└ {:?}", err));
                    return;
                }
            };
            body_msg
                .iter()
                .filter(|msg| ids.contains(&msg.author.id))
                .map(|msg| msg.id)
                .collect::<Vec<_>>()
        }
        Err(err) => {
            error(&format!("Failed to fetch messages\n└ {:?}", err));
            return;
        }
    };

    match ctx
        .http
        .delete_messages(channel.id, messages.as_slice())
        .await
    {
        Ok(_) => success = ids.iter().map(|id| id).collect(),
        Err(err) => {
            error(&format!("Failed to delete messages\n└ {:?}", err));
            failed = ids.iter().map(|id| id).collect();
        }
    }

    let embed = officer_cui_action_embed(user, "Delete messages", &success, &failed, "");

    let payload = InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL)
        .embeds(vec![embed])
        .build();

    update_reply(ctx, interaction, payload).await;
}
