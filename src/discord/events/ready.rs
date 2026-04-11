use crate::constants::EMOJIS;
use crate::{discord::*, functions::*};
use colored::Colorize;
use std::error::Error;
use twilight_gateway::{Event, EventType};
use twilight_model::gateway::payload::outgoing::UpdatePresence;
use twilight_model::gateway::presence::{Activity, ActivityEmoji, ActivityType, Status};

pub struct Ready;

#[async_trait]
impl EventHandler for Ready {
    fn event(&self) -> EventType {
        EventType::Ready
    }

    async fn run(&self, ctx: Context, event: Event) -> Result<(), Box<dyn Error + Send + Sync>> {
        let ready = match event {
            Event::Ready(e) => e,
            _ => return Ok(()),
        };

        log(&format!("{} {} {}", "●".bright_green(), ready.user.name.underline().bright_green(), "online ✓".bright_green()));

        let app_id = ready.application.id;
        let commands = HANDLERS
            .app_command_handlers
            .values()
            .map(|val| val.command())
            .collect::<Vec<_>>();

        let res = ctx
            .http
            .interaction(app_id)
            .set_global_commands(&commands)
            .await;

        match res {
            Ok(_) => {
                log(
                    &format!(
                        "{} {} {}",
                        "└".bright_green(),
                        commands.len().to_string().bright_green(),
                        "command(s) successfully registered globally!".bright_green()
                    )
                    .bright_green(),
                );
                for command in commands {
                    log(
                        &format!(
                            "{} > {} {}",
                            "{/} Slash command".bright_green(),
                            command.name.as_str().underline().bright_blue(),
                            "✓".bright_green()
                        )
                        .bright_green(),
                    );
                }
            }
            Err(err) => {
                error(&format!(
                    "└ {} command(s) successfully registered globally!",
                    commands.len()
                ));
                error(&format!("{:?}", err));
                return Err(Box::new(err));
            }
        }

        for handler in HANDLERS.message_component_handlers.iter() {
            let id = handler.0.as_str();
            log(&format!("{} > {} {}", "▸ message component".bright_green(), id.underline().bright_blue(), "✓".bright_green()));
        }

        for handler in HANDLERS.modal_handlers.iter() {
            let id = handler.0.as_str();
            log(&format!("{} > {} {}", "▸ modal".bright_green(), id.underline().bright_blue(), "✓".bright_green()));
        }

        for handler in HANDLERS.event_handlers.iter() {
            if let Some(event_name) = handler.0.name() {
                log(&format!("{} > {}", "☉ event".bright_yellow(), event_name.underline().bright_yellow()));
            }
        }

        let activity = Activity {
            application_id: None,
            assets: None,
            buttons: vec![],
            created_at: None,
            details: None,
            emoji: None,
            flags: None,
            id: None,
            instance: None,
            kind: ActivityType::Custom,
            name: "Custom Status".to_string(),
            party: None,
            secrets: None,
            state: Some("Rust-powered bot.\nHosted by discloud.".to_string()),
            timestamps: None,
            url: None,
        };

        if let Ok(presence) = UpdatePresence::new(vec![activity], false, None, Status::Online) {
            ctx.sender.command(&presence)?;
        }

        Ok(())
    }
}
