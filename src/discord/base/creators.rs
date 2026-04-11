use crate::discord::Context;
pub use async_trait::async_trait;
use std::error::Error;
use twilight_gateway::{Event, EventType};
use twilight_model::application::command::*;
use twilight_model::gateway::payload::incoming::{InteractionCreate, MessageCreate};

#[async_trait]
pub trait EventHandler: Send + Sync {
    fn event(&self) -> EventType;
    async fn run(&self, ctx: Context, event: Event) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait PrefixCommandHandler: Send + Sync {
    fn name(&self) -> String;
    async fn run(
        &self,
        ctx: Context,
        message: Box<MessageCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait SlashCommandHandler: Send + Sync {
    fn command(&self) -> Command;
    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait ModalHandler: Send + Sync {
    fn custom_id(&self) -> String;
    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

#[async_trait]
pub trait MessageComponentHandler: Send + Sync {
    fn custom_id(&self) -> String;
    async fn run(
        &self,
        ctx: Context,
        interaction: &Box<InteractionCreate>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
