use crate::discord::{Context, ModalHandler};
use crate::functions::{
    defer_reply, error, get_modal_data, modal_labels_to_hash, officer_cui_action_embed,
    update_reply,
};
use async_trait::async_trait;
use std::error::Error;
use std::hash::Hash;
use twilight_model::application::interaction::modal::ModalInteractionComponent;
use twilight_model::channel::message::MessageFlags;
use twilight_model::gateway::payload::incoming::InteractionCreate;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_util::builder::InteractionResponseDataBuilder;

pub struct Unban;

#[async_trait]
impl ModalHandler for Unban {
    fn custom_id(&self) -> String {
        "modal/moderate/unban".into()
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
            let ModalInteractionComponent::StringSelect(menu) = component else {
                error("Failed to parse user field.");
                return Ok(());
            };
            menu.values
                .iter()
                .map(|val| {
                    let id_u64 = val.parse::<u64>().unwrap_or(0);

                    Id::<UserMarker>::new(id_u64)
                })
                .filter(|val| val.get() != 0)
                .collect::<Vec<_>>()
        } else {
            error("Failed to parse user field.");
            return Ok(());
        };

        unban_action(&ctx, interaction, &users).await;

        Ok(())
    }
}

pub async fn unban_action(
    ctx: &Context,
    interaction: &Box<InteractionCreate>,
    ids: &[Id<UserMarker>],
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
        match ctx.http.delete_ban(guild_id.clone(), id.clone()).await {
            Ok(_) => success.push(id),
            Err(err) => {
                error(&format!("Failed to unban user\n└ {:?}", err));
                failed.push(id)
            }
        }
    }

    let embed = officer_cui_action_embed(user, "Unban", &success, &failed, "");

    let payload = InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL)
        .embeds(vec![embed])
        .build();

    update_reply(ctx, interaction, payload).await;
}
