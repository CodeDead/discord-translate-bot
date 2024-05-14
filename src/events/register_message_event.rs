use crate::commands;
use log::{error, info};
use serenity::all::{Context, CreateMessage, Message};

/// Handle the register message event
///
/// # Arguments
///
/// * `ctx` - The context of the event
/// * `msg` - The message of the event
pub async fn handle(ctx: &Context, msg: &Message) {
    let guild_id = msg.guild_id;

    if let Some(guild) = guild_id {
        let commands = guild
            .set_commands(&ctx.http, vec![commands::translate_text::register()])
            .await;

        info!("Registered commands {commands:#?} for guild {:?}", guild);
        let builder = CreateMessage::new().reference_message(msg).content(
            "Commands registered! Try using /translate-text to translate a message to English.",
        );

        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            error!("Error sending message: {:?}", why);
        }
    }
}
