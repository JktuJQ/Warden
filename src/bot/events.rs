//! `events` module contains event handler for bot.
//!

use crate::{
    bot::music_playing::*,
    db::{
        connections::SERVERS_DB,
        models::{Id, Setting},
    },
    logger,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        gateway::Ready,
        guild::Guild,
        id::{ChannelId, GuildId},
    },
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

pub async fn check_music_log_channel(guild_id: GuildId, channel_id: ChannelId) -> bool {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let setting: Option<Setting> = sqlx::query_as::<_, Setting>("SELECT music_log_channel_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)").bind(guild_id.to_string()).fetch_optional(connection).await.expect("Query should be correct");
    if let Some(setting) = setting {
        if let Some(music_log_channel_id) = setting.music_log_channel_id.0 {
            if music_log_channel_id == channel_id.0 {
                return true;
            }
        }
    }
    false
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

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_none()
            || !check_music_log_channel(
                message.guild_id.expect("First condition exclusives None"),
                message.channel_id,
            )
            .await
            || !message.content.starts_with("music1 ")
        {
            return;
        }
        let command: Vec<&str> = message.content.split_whitespace().collect::<Vec<&str>>();
        match command[1] {
            "play" => {
                play(&ctx, &message, command[2]).await;
            }
            "join" => {
                join(
                    &ctx,
                    &message,
                    ChannelId(
                        command[2]
                            .parse::<u64>()
                            .expect("Correct id should be given"),
                    ),
                )
                .await
            }
            "leave" => leave(&ctx, &message).await,
            "pause" => pause(&ctx, &message).await,
            "resume" => resume(&ctx, &message).await,
            "skip" => skip(&ctx, &message).await,
            "stop" => stop(&ctx, &message).await,
            _ => unreachable!("All outcomes should be covered"),
        };
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

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_none()
            || !check_music_log_channel(
                message.guild_id.expect("First condition exclusives None"),
                message.channel_id,
            )
            .await
            || !message.content.starts_with("music2 ")
        {
            return;
        }
        let command: Vec<&str> = message.content.split_whitespace().collect::<Vec<&str>>();
        match command[1] {
            "play" => {
                play(&ctx, &message, command[2]).await;
            }
            "join" => {
                join(
                    &ctx,
                    &message,
                    ChannelId(
                        command[2]
                            .parse::<u64>()
                            .expect("Correct id should be given"),
                    ),
                )
                .await
            }
            "leave" => leave(&ctx, &message).await,
            "pause" => pause(&ctx, &message).await,
            "resume" => resume(&ctx, &message).await,
            "skip" => skip(&ctx, &message).await,
            "stop" => stop(&ctx, &message).await,
            _ => unreachable!("All outcomes should be covered"),
        };
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

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_none()
            || !check_music_log_channel(
                message.guild_id.expect("First condition exclusives None"),
                message.channel_id,
            )
            .await
            || !message.content.starts_with("music3 ")
        {
            return;
        }
        let command: Vec<&str> = message.content.split_whitespace().collect::<Vec<&str>>();
        match command[1] {
            "play" => {
                play(&ctx, &message, command[2]).await;
            }
            "join" => {
                join(
                    &ctx,
                    &message,
                    ChannelId(
                        command[2]
                            .parse::<u64>()
                            .expect("Correct id should be given"),
                    ),
                )
                .await
            }
            "leave" => leave(&ctx, &message).await,
            "pause" => pause(&ctx, &message).await,
            "resume" => resume(&ctx, &message).await,
            "skip" => skip(&ctx, &message).await,
            "stop" => stop(&ctx, &message).await,
            _ => unreachable!("All outcomes should be covered"),
        };
    }
}
