use crate::events::{
    help_message_event, register_message_event, translate_message_event, unregister_message_event,
};
use crate::services::google_translate_service::GoogleTranslateService;
use log::info;
use serenity::all::{Context, EventHandler, Interaction, Message, Ready};
use serenity::async_trait;

pub struct Handler {
    google_translate_service: GoogleTranslateService,
}

impl Handler {
    /// Create a new Handler instance
    ///
    /// # Arguments
    ///
    /// * `google_translate_service` - The Google Translate service
    ///
    /// # Returns
    ///
    /// A new Handler instance
    pub fn new(google_translate_service: GoogleTranslateService) -> Self {
        Self {
            google_translate_service,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    /// Handle the message event
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the event
    /// * `msg` - The message of the event
    async fn message(&self, ctx: Context, msg: Message) {
        let command = msg.content.split_whitespace().next().unwrap_or("");
        info!("Received message: {command}");

        match command {
            "!translate" => {
                translate_message_event::handle(&ctx, &msg, &self.google_translate_service).await;
            }
            "!register" => {
                register_message_event::handle(&ctx, &msg).await;
            }
            "!unregister" => {
                unregister_message_event::handle(&ctx, &msg).await;
            }
            "!help" => {
                help_message_event::handle(&ctx, &msg).await;
            }
            _ => {}
        }
    }

    /// Handle the ready event
    ///
    /// # Arguments
    ///
    /// * `ctx` - The context of the event
    /// * `ready` - The ready event
    async fn ready(&self, _ctx: Context, ready: Ready) {
        ready.guilds.iter().for_each(|guild| {
            info!("{} is connected to guild: {}", ready.user.name, guild.id);
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!("Received command interaction: {command:#?}");

            if command.data.name.as_str() == "translate-text" {
                crate::interactions::translate_text_interaction::handle(
                    &ctx,
                    &command,
                    &self.google_translate_service,
                )
                .await;
            }
        }
    }
}
