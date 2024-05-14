use crate::models::config_error::ConfigError;
use log::{error, info};
use std::env;
use std::error::Error;

pub struct Config {
    pub google_translate_token: String,
    pub discord_token: String,
}

impl Config {
    /// Create a new Config instance
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError` if GOOGLE_TRANSLATE_TOKEN or DISCORD_TOKEN is not set
    pub fn new() -> Result<Self, Box<dyn Error>> {
        info!("Loading configuration");

        let google_translate_token =
            env::var("GOOGLE_TRANSLATE_TOKEN").expect("GOOGLE_TRANSLATE_TOKEN must be set");
        let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");

        if google_translate_token.trim().is_empty() {
            error!("GOOGLE_TRANSLATE_TOKEN must not be empty");
            return Err(Box::new(ConfigError::new(
                "GOOGLE_TRANSLATE_TOKEN must not be empty",
            )));
        }

        if discord_token.trim().is_empty() {
            error!("DISCORD_TOKEN must not be empty");
            return Err(Box::new(ConfigError::new(
                "DISCORD_TOKEN must not be empty",
            )));
        }

        Ok(Self {
            google_translate_token,
            discord_token,
        })
    }
}
