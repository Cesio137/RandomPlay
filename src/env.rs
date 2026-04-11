use crate::functions::*;
use colored::Colorize;
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
pub struct EnvSchema {
    pub BOT_TOKEN: String,
    pub DISCLOUD_TOKEN: String,
    pub GEMINI_API_KEY: String,
}

pub static ENV: Lazy<EnvSchema> = Lazy::new(|| {
    if let Err(err) = dotenv() {
        if let Err(e) = dotenvy::from_filename("./packages/bot/.env") {
            error(&format!("Failed to load .env file\n└{}", err));
            panic!();
        }
    }

    let env_vars = env::vars().collect::<HashMap<String, String>>();

    let env: EnvSchema = match serde_json::to_string(&env_vars) {
        Ok(env_str) => match serde_json::from_str(&env_str) {
            Ok(schema) => schema,
            Err(err) => {
                error(&format!("Failed to parse environment variables\n└{}", err));
                panic!();
            }
        },
        Err(err) => {
            error(&format!("Failed to parse environment variables\n└{}", err));
            panic!();
        }
    };

    log(&format!("{} {}", "☰ Environment variables".bright_purple(), "loaded ✓".bright_purple()));

    env
});
