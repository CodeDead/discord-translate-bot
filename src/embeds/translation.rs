use serenity::all::{CreateEmbed, CreateEmbedFooter, Timestamp};

/// Get an embed for a translation
///
/// # Arguments
///
/// * `original_message` - The original message
/// * `translation` - The translated message
/// * `target` - The target language
///
/// # Returns
///
/// A `CreateEmbed` instance
pub fn get_embed(original_message: &str, translation: &str, target: &str) -> CreateEmbed {
    let translation_title = format!("Translation ({})", target);
    let footer = CreateEmbedFooter::new("Translated by Google Translate");
    CreateEmbed::new()
        .fields(vec![
            ("Original", original_message, false),
            (&translation_title, &translation, true),
        ])
        .footer(footer)
        .timestamp(Timestamp::now())
        .url("https://codedead.com/donate")
}
