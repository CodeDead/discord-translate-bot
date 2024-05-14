use crate::services::google_translate_service::GoogleTranslateService;
use crate::{commands, embeds};
use log::error;
use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
    ResolvedOption, ResolvedValue,
};

/// Handle the `translate-text` interaction
///
/// # Arguments
///
/// * `ctx` - The context of the event
/// * `command` - The command interaction
/// * `google_translate_service` - The Google Translate service
pub async fn handle(
    ctx: &Context,
    command: &CommandInteraction,
    google_translate_service: &GoogleTranslateService,
) {
    let options = command.data.options();

    let mut message = "";
    let mut target = "en";

    if let Some(ResolvedOption {
        value: ResolvedValue::String(msg),
        ..
    }) = options.first()
    {
        message = msg;
    } else {
        let embed = embeds::error::get_embed("No message provided");
        let data = CreateInteractionResponseMessage::new().embed(embed);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            error!("Cannot respond to slash command: {why}");
        }
    }

    if options.len() > 1 {
        if let Some(ResolvedOption {
            value: ResolvedValue::String(mgs),
            ..
        }) = options.last()
        {
            target = mgs;
        }
    }

    let res = commands::translate_text::run(&message, &target, google_translate_service)
        .await
        .unwrap_or_else(|e| {
            error!("Error translating message: {:?}", e);
            embeds::error::get_embed(&e.message)
        });

    let data = CreateInteractionResponseMessage::new().embed(res);
    let builder = CreateInteractionResponse::Message(data);
    if let Err(why) = command.create_response(&ctx.http, builder).await {
        error!("Cannot respond to slash command: {why}");
    }
}
