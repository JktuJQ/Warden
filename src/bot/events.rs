//! `events` module contains event handler for bot.
//!

use crate::{
    db::{connections::SERVERS_DB, models::Id},
    logger,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{gateway::Ready, guild::Guild},
};
use sqlx::SqlitePool;

/// `Handler` struct implements `EventHandler` trait for main bot.
///
pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        logger::log(
            log::Level::Info,
            &format!("{} is connected!", ready.user.name),
        );
    }

    async fn guild_create(&self, _: Context, guild: Guild, _: bool) {
        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this point");

        let guild_id: Id = guild.id.into();
        if sqlx::query("SELECT discord_id FROM guilds WHERE discord_id = ?")
            .bind(guild_id.to_string())
            .fetch_optional(connection)
            .await
            .expect("Query should be correct")
            .is_some()
        {
            logger::log(log::Level::Info, &format!("On {} guild ready", guild.name));
            return;
        }
        sqlx::query(
            "
            INSERT INTO settings VALUES (NULL, NULL, NULL, NULL);
            INSERT INTO guilds VALUES (?, (SELECT last_insert_rowid()));
        ",
        )
        .bind(guild_id.to_string())
        .execute(connection)
        .await
        .expect("Query should be correct");
        logger::log(
            log::Level::Info,
            &format!("Registered {} guild", guild.name),
        );
        logger::log(log::Level::Info, &format!("On {} guild ready", guild.name));
    }
}


pub struct MusicHandler1;
#[async_trait]
impl EventHandler for MusicHandler1 {
    async fn ready(&self, _: Context, ready: Ready) {
        logger::log(
            log::Level::Info,
            &format!("{} is connected!", ready.user.name),
        );
    }
}
pub struct MusicHandler2;
#[async_trait]
impl EventHandler for MusicHandler2 {
    async fn ready(&self, _: Context, ready: Ready) {
        logger::log(
            log::Level::Info,
            &format!("{} is connected!", ready.user.name),
        );
    }
}
pub struct MusicHandler3;
#[async_trait]
impl EventHandler for MusicHandler3 {
    async fn ready(&self, _: Context, ready: Ready) {
        logger::log(
            log::Level::Info,
            &format!("{} is connected!", ready.user.name),
        );
    }
}