//! Handles connection to and interaction with the database.
use serenity::model::channel::Message;
use tokio_postgres::{Client, Error, NoTls, Statement};

const CREATE_STATEMENT: &str = "CREATE TABLE IF NOT EXISTS messages (
    id BIGINT PRIMARY KEY,
    guild BIGINT NOT NULL,
    author BIGINT NOT NULL,
    channel BIGINT NOT NULL,
    reply_to BIGINT,
    timestamp TIMESTAMP NOT NULL
);";
const INSERT_STATEMENT: &str = "INSERT INTO messages (
    id, guild, author, channel, reply_to, timestamp
) VALUES ($1, $2, $3, $4, $5, $6);";

/// The connection to the database and cached statements.
pub struct DbConnection {
    client: Client,
    insert_statement: Statement,
}

/// Dump a Discord (Serenity) message to the database.)
pub async fn store_message(db: &DbConnection, message: Message) -> Result<(), Error> {
    let id = *message.id.as_u64() as i64;
    let guild = message.guild_id.map(|id| *id.as_u64() as i64);
    let author = *message.author.id.as_u64() as i64;
    let channel = *message.channel_id.as_u64() as i64;
    let reply_to = message
        .referenced_message
        .map(|message| *message.author.id.as_u64() as i64);
    let timestamp = message.timestamp.naive_utc();
    db.client
        .execute(
            &db.insert_statement,
            &[&id, &guild, &author, &channel, &reply_to, &timestamp],
        )
        .await?;
    Ok(())
}

/// Connect to the database, create the table and prepare statements.
pub async fn init_db(db_url: String) -> Result<DbConnection, Error> {
    let (postgres, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    postgres.execute(CREATE_STATEMENT, &[]).await?;
    let insert_statement = postgres.prepare(INSERT_STATEMENT).await?;
    Ok(DbConnection {
        client: postgres,
        insert_statement,
    })
}
