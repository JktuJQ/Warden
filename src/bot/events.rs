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
}
