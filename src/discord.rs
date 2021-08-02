//! Handles interaction with Discord.
use crate::database::{store_message, DbConnection};
use serenity::{
    async_trait,
    client::bridge::gateway::GatewayIntents,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

/// Handles incoming events from Discord and stores constant globals.
struct DiscordHandler {
    db: DbConnection,
}

#[async_trait]
impl EventHandler for DiscordHandler {
    /// Handle an incoming message and store it in the database.
    async fn message(&self, _ctx: Context, msg: Message) {
        store_message(&self.db, msg)
            .await
            .expect("Error while storing message");
    }

    /// Log to the console once the service is running.
    async fn ready(&self, _ctx: Context, _ready: Ready) {
        println!("Successfully connected to Discord and Postgres.");
    }
}

/// Connect to Discord and set up the event handler.
pub async fn init_discord(db: DbConnection, token: String) {
    let mut discord = Client::builder(token)
        .event_handler(DiscordHandler { db })
        .intents(GatewayIntents::GUILD_MESSAGES)
        .await
        .expect("Error creating client");
    if let Err(why) = discord.start().await {
        println!("Discord client error: {:?}", why);
    }
}
