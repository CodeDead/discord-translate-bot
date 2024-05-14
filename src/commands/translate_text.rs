use crate::models::google_translate_error::GoogleTranslateError;
use crate::services::google_translate_service::GoogleTranslateService;
use log::error;
use serenity::all::CreateEmbed;
use serenity::builder::{CreateCommand, CreateCommandOption};

/// Register the translate-text command
///
/// # Returns
///
/// A `CreateCommand` instance
pub fn register() -> CreateCommand {
    CreateCommand::new("translate-text")
        .description("Translate a message to English")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "message",
                "The message to translate",
            )
            .required(true),
        )
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "target",
                "The target language",
            )
            .required(false),
        )
}

/// Run the translate-text command
///
/// # Arguments
///
/// * `message` - The message to translate
/// * `target` - The target language
/// * `google_translate_service` - The Google Translate service
///
/// # Returns
///
/// A `Result` containing the translated message or a `GoogleTranslateError` instance
pub async fn run(
    message: &str,
    target: &str,
    google_translate_service: &GoogleTranslateService,
) -> Result<CreateEmbed, GoogleTranslateError> {
    let translation = match google_translate_service.translate(message, target).await {
        Ok(t) => t,
        Err(err) => {
            error!("Error translating message: {:?}", err.message);
            return Err(err);
        }
    };

    Ok(crate::embeds::translation::get_embed(
        message,
        &translation,
        target,
    ))
}
