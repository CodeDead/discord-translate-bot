use crate::embeds;
use crate::services::google_translate_service::GoogleTranslateService;
use log::error;
use serenity::all::{Context, CreateMessage, Message};

/// Handle the translation message event
///
/// # Arguments
///
/// * `ctx` - The context of the event
/// * `msg` - The message of the event
/// * `google_translate_service` - The Google Translate service
pub async fn handle(
    ctx: &Context,
    msg: &Message,
    google_translate_service: &GoogleTranslateService,
) {
    if msg.referenced_message.is_none() {
        return;
    }

    let options = msg
        .content
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>();

    let target = options.first().unwrap_or(&"en");

    let original_referenced_message = msg.clone().referenced_message.unwrap().content;
    let embed = match google_translate_service
        .translate(&original_referenced_message, target)
        .await
    {
        Ok(translation) => {
            embeds::translation::get_embed(&original_referenced_message, &translation, target)
        }
        Err(why) => {
            error!("Error translating message: {:?}", why.message);
            // Return an error embed
            embeds::error::get_embed(&why.message)
        }
    };

    let builder = CreateMessage::new().reference_message(msg).embed(embed);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        error!("Error sending message: {:?}", why);
    }
}
