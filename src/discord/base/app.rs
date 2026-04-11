use crate::discord::HANDLERS;
use crate::functions::{log, warn};
use crate::{env::ENV, functions::error};
use std::sync::Arc;
use tokio::task::JoinSet;
use tokio::time::{Duration, sleep};
use twilight_cache_inmemory::{DefaultInMemoryCache, InMemoryCache, ResourceType};
use twilight_gateway::{
    Config, ConfigBuilder, Event, EventTypeFlags, Intents, MessageSender, Shard,
    StreamExt as _,
};
use twilight_http::Client;
use twilight_http_ratelimiting::RateLimiter;
use twilight_model::application::interaction::InteractionData;

pub struct App {
    pub config: Config,
    pub http: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
}

#[derive(Clone)]
pub struct Context {
    pub http: Arc<Client>,
    pub cache: Arc<InMemoryCache>,
    pub sender: MessageSender,
}

impl App {
    pub async fn bootstrap(intents: Intents) -> Self {
        let token = &ENV.BOT_TOKEN;

        // Create HTTP client with rate limiting to prevent 429 errors
        let http = Client::builder()
            .token(token.clone())
            .ratelimiter(Some(RateLimiter::default()))
            .build();
        let http = Arc::new(http);

        let config = Config::new(token.clone(), intents);

        let cache = Arc::new(
            DefaultInMemoryCache::builder()
                .resource_types(ResourceType::all())
                .build(),
        );

        Self {
            config,
            http,
            cache,
        }
    }

    pub async fn run(&mut self) {
        let config_callback = |_, builder: ConfigBuilder| builder.build();

        let mut shards = match twilight_gateway::create_recommended(
            &self.http,
            self.config.clone(),
            config_callback,
        )
        .await
        {
            Ok(shards) => shards,
            Err(err) => {
                error(&format!("Error trying to create shards\n└ {:?}", err));
                panic!();
            }
        }
        .collect::<Vec<_>>();

        const RESHARD_DURATION: Duration = Duration::from_secs(60 * 60 * 8);

        loop {
            let mut set = JoinSet::new();
            for shard in shards {
                set.spawn(App::shard_handle(
                    self.http.clone(),
                    self.cache.clone(),
                    shard,
                ));
            }

            let reshard_timer = sleep(RESHARD_DURATION);
            tokio::pin!(reshard_timer);

            let mut has_errors = false;
            loop {
                tokio::select! {
                    result = set.join_next() => {
                        match result {
                            Some(Ok(_)) => log("Shard finished successfully."),
                            Some(Err(e)) => {
                                error(&format!("Shard task error (panic/cancel): {:?}", e));
                                has_errors = true;
                            }
                            None => {
                                warn("All shards finished before the 1h timer.");
                                break;
                            }
                        }
                    }

                    _ = &mut reshard_timer => {
                        log("8 hours passed, initiating reshard...");
                        break;
                    }
                }
            }

            if has_errors {
                warn("Errors were detected during the shard session.");
            }

            shards = match twilight_gateway::create_recommended(
                &self.http,
                self.config.clone(),
                config_callback,
            )
            .await
            {
                Ok(shards) => shards,
                Err(err) => {
                    error(&format!("Error trying to create shards\n└ {:?}", err));
                    panic!();
                }
            }
            .collect::<Vec<_>>();

            set.abort_all();
        }
    }

    pub async fn shard_handle(http: Arc<Client>, cache: Arc<InMemoryCache>, mut shard: Shard) {
        let ctx = Context {
            http,
            cache,
            sender: shard.sender(),
        };

        while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
            let Ok(event) = item else {
                error(&format!("Error receiving event\n└ {:?}", item.unwrap_err()));
                continue;
            };

            // Update the cache with the event.
            ctx.cache.update(&event);

            tokio::spawn(App::handle_event(ctx.clone(), event));
        }
    }

    async fn handle_event(ctx: Context, event: Event) {
        if let Some(callback) = HANDLERS.event_handlers.get(&event.kind()) {
            if let Err(err) = callback.run(ctx, event).await {
                error(&format!("Event error!\n└ {:?}", err));
            }
            return;
        }

        match event {
            Event::InteractionCreate(interaction) => {
                if let Some(data) = &interaction.data {
                    match data {
                        InteractionData::ApplicationCommand(command) => {
                            if let Some(callback) =
                                HANDLERS.app_command_handlers.get(command.name.as_str())
                            {
                                if let Err(err) = callback.run(ctx, &interaction).await {
                                    error(&format!("Application command error!\n└ {:?}", err));
                                }
                            }
                        }
                        InteractionData::ModalSubmit(modal_data) => {
                            if let Some(callback) =
                                HANDLERS.modal_handlers.get(modal_data.custom_id.as_str())
                            {
                                if let Err(err) = callback.run(ctx, &interaction).await {
                                    error(&format!("Modal submit error!\n└ {:?}", err));
                                }
                            }
                        }
                        InteractionData::MessageComponent(message_component) => {
                            if let Some(callback) = HANDLERS
                                .message_component_handlers
                                .get(message_component.custom_id.as_str())
                            {
                                if let Err(err) = callback.run(ctx, &interaction).await {
                                    error(&format!("Modal submit error!\n└ {:?}", err));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
