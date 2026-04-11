mod constants;
mod discord;
mod env;
mod functions;
mod menus;
mod models;
mod tools;

#[cfg(target_env = "gnu")]
use crate::functions::configure_malloc;

use crate::discord::*;
use twilight_model::gateway::Intents;

#[tokio::main]
async fn main() {
    #[cfg(target_env = "gnu")]
    configure_malloc();

    let intents = Intents::all();

    let mut app = App::bootstrap(intents).await;
    app.run().await;
}
