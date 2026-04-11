use crate::discord::{Context, ModalHandler};
use crate::functions::filters::filter_users_from_mod_action;
use crate::functions::{
    defer_reply, error, get_modal_data, modal_labels_to_hash, officer_cui_action_embed,
    update_reply,
};
use async_trait::async_trait;
use std::error::Error;
use twilight_http::request::AuditLogReason;
use twilight_model::application::interaction::modal::ModalInteractionComponent;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct Kick;

#[async_trait]
impl ModalHandler for Kick {
    fn custom_id(&self) -> String {
        "modal/moderate/kick".into()
    }

    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let Some(modal_data) = get_modal_data(interaction) else {
            error("Failed to parse modal data");
            return Ok(());
        };

        let Some(guild_id) = &interaction.guild_id else {
            error("Failed to get guild from interaction");
            return Ok(());
        };

        defer_reply(&ctx, interaction, true).await;

        let modal_components = modal_labels_to_hash(&modal_data.components);

        let users = if let Some(component) = modal_components.get("users") {
            let ModalInteractionComponent::UserSelect(menu) = component else {
                error("Failed to parse user field.");
                return Ok(());
            };
            filter_users_from_mod_action(&ctx, guild_id, menu.values.as_slice()).await?
        } else {
            error("Failed to parse user field.");
            return Ok(());
        };

        let reason = if let Some(component) = modal_components.get("reason") {
            match &component {
                ModalInteractionComponent::TextInput(modal_interaction_text_input) => {
                    modal_interaction_text_input.value.as_str()
                }
                _ => "",
            }
        } else {
            error("Failed to parse reason field.");
            return Ok(());
        };

        kick_action(&ctx, interaction, &users, reason).await;

        Ok(())
    }
}

pub async fn kick_action(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    ids: &[Id<UserMarker>],
    reason: &str,
) {
    let Some(guild_id) = interaction.guild_id.as_ref() else {
        error("Error trying to acess unknow guild.");
        return;
    };

    let user = match interaction.member.as_ref() {
        Some(member) => {
            if let Some(user) = member.user.as_ref() {
                user
            } else {
                error("Error trying to acess unknow user.");
                return;
            }
        }
        None => {
            error("Error trying to acess unknow member.");
            return;
        }
    };

    let mut success = Vec::new();
    let mut failed = Vec::new();

    for id in ids {
        match ctx
            .http
            .remove_guild_member(guild_id.clone(), id.clone())
            .reason(reason)
            .await
        {
            Ok(_) => success.push(id),
            Err(err) => {
                error(&format!("Failed to kick user\n└ {:?}", err));
                failed.push(id)
            }
        }
    }

    let embed = officer_cui_action_embed(user, "Kick", &success, &failed, reason);

    let payload = InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL)
        .embeds(vec![embed])
        .build();

    update_reply(ctx, interaction, payload).await;
}
