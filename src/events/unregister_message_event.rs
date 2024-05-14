use log::{error, info};
use serenity::all::{Context, CreateMessage, Message};

/// Handle the unregister message event
///
/// # Arguments
///
/// * `ctx` - The context of the event
/// * `msg` - The message of the event
pub async fn handle(ctx: &Context, msg: &Message) {
    let guild_id = msg.guild_id;

    if let Some(guild) = guild_id {
        let commands = guild.set_commands(&ctx.http, vec![]).await;

        info!("Unregistered commands {commands:#?} for guild {:?}", guild);
        let builder = CreateMessage::new()
            .reference_message(msg)
            .content("Commands removed!");

        if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
            error!("Error sending message: {:?}", why);
        }
    }
}
