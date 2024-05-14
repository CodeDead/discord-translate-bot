use serenity::all::{CreateEmbed, CreateEmbedFooter, Timestamp};

/// Get an error embed
///
/// # Arguments
///
/// * `message` - The error message
///
/// # Returns
///
/// A `CreateEmbed` instance
pub fn get_embed(message: &str) -> CreateEmbed {
    let footer = CreateEmbedFooter::new("Copyright © 2024 CodeDead");
    CreateEmbed::new()
        .title("Error")
        .description("An error occurred while translating the message")
        .field("Something went wrong", message, false)
        .footer(footer)
        .timestamp(Timestamp::now())
        .url("https://github.com/CodeDead/discord-translate-bot/issues")
}
