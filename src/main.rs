mod commands;
mod config;
mod embeds;
mod events;
mod interactions;
mod models;
mod services;

use crate::config::configuration::Config;
use crate::events::event_handler::Handler;
use crate::services::google_translate_service::GoogleTranslateService;
use log::error;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(_) => {},
        Err(e) => println!("Could not load .env file: {e}"),
    };

    env_logger::init();

    let config = Config::new().expect("Failed to load configuration");
    let handler = Handler::new(GoogleTranslateService::new(&config.google_translate_token));

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&config.discord_token, intents)
        .event_handler(handler)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
