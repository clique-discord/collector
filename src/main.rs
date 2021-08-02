//! A service which connects to Discord and stores message metadata in a database.
use configparser::ini::Ini;
use std::error::Error;

mod database;
mod discord;
use database::init_db;
use discord::init_discord;

/// Parse the config file and start the database and discord connections.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Ini::new();
    config
        .load("config.ini")
        .expect("File 'config.ini' not found.");
    let token = config
        .get("clique-collector", "discord_token")
        .expect("Key 'discord-token' missing in map 'clique-collector' of config.ini.");
    let db_url = config
        .get("clique-collector", "postgres_url")
        .expect("Key 'postgres-url' missing in map 'clique-collector of config.ini.");
    let db = init_db(db_url).await?;
    init_discord(db, token).await;
    Ok(())
}
