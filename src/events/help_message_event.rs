use log::error;
use serenity::all::{Context, CreateMessage, Message};

/// Handle the help message event
///
/// # Arguments
///
/// * `ctx` - The context of the event
/// * `msg` - The message of the event
pub async fn handle(ctx: &Context, msg: &Message) {
    let content = "Hello! I'm a bot that can translate messages.\n\nTo translate a message, use the `/translate-text` command. For example, `/translate-text Hello!` will translate the message `Hello!` to English. You can also use the `/translate-text` command with a target language. For example, `/translate-text Hello! fr` will translate the message `Hello!` to French. You can find a list of supported languages [here](https://cloud.google.com/translate/docs/languages).\
    \n\nIf you want to translate a message in a channel or thread, reply to the message with `!translate` followed (optionally) by the target language. For example, `!translate fr` will translate the message to French.\
    \n\nIf you want to register the `/translate-text` command in your server, use the `!register` command.\
    \n\nIf you want to unregister the `/translate-text` command in your server, use the `!unregister` command.\
    \n\nI was created by [CodeDead](https://codedead.com/) using [Serenity](https://github.com/serenity-rs/serenity). If you happen to find a bug, feel free to let me know [here](https://github.com/CodeDead/discord-translate-bot/issues)!";

    let builder = CreateMessage::new().reference_message(msg).content(content);

    if let Err(why) = msg.channel_id.send_message(&ctx.http, builder).await {
        error!("Error sending message: {:?}", why);
    }
}
