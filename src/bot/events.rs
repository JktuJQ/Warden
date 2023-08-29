//! `events` module contains event handler for bot.
//!

use crate::{
    bot::music_playing::*,
    db::{
        connections::SERVERS_DB,
        models::{Id, Setting, UnregisteredMember},
    },
    logger,
};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::Message,
        gateway::Ready,
        guild::{Guild, Member, UnavailableGuild},
        id::{ChannelId, GuildId, RoleId, UserId},
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
            INSERT INTO settings VALUES (NULL, NULL, NULL, NULL, NULL, NULL);
            INSERT INTO guilds VALUES (?, (SELECT last_insert_rowid()));
            INSERT INTO music_bots VALUES (?, 'music1 ', NULL), (?, 'music2 ', NULL), (?, 'music3 ', NULL);
        ",
        )
        .bind(guild_id.to_string())
        .bind(guild_id.to_string())
        .bind(guild_id.to_string())
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

    async fn guild_delete(&self, _: Context, incomplete: UnavailableGuild, _: Option<Guild>) {
        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this point");

        sqlx::query(
            "DELETE FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)",
        )
        .bind(incomplete.id.to_string())
        .execute(connection)
        .await
        .expect("Query should be correct");
        logger::log(
            log::Level::Info,
            &format!("Unregistered MAIN_BOT from '{}' guild", incomplete.id),
        );
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this moment");

        logger::log_discord(
            &ctx.http,
            member.guild_id,
            "guild_member_addition was called",
        )
        .await;

        let message: String = format!("Welcome to '{}' server!\nPlease, dm me your real name in following form -> '-name <your_name>', e.g. '-name Ваня'", member.guild_id.name(&ctx.cache).expect("This can be called only on guild"));
        if let Err(why) = member.user.dm(&ctx, |m| m.content(&message)).await {
            let Setting { id: _, moderation_channel_id: _, log_channel_id: _, music_order_channel_id: _, music_log_channel_id: _, member_role_id } = sqlx::query_as::<_, Setting>("SELECT member_role_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)").bind(member.guild_id.to_string()).fetch_one(connection).await.expect("Query should be correct");
            let member_role_id: RoleId = RoleId::from(Id(member_role_id
                .0
                .expect("member_role_id should be set at this moment")));
            let member_roles: Vec<RoleId> = {
                let mut vec: Vec<RoleId> = member.roles;
                vec.push(member_role_id);
                vec
            };
            let _ = member
                .guild_id
                .edit_member(&ctx.http, member.user.id, |m| {
                    m.roles(&member_roles)
                        .nickname(format!("<{}>", member.user.name))
                })
                .await;
            logger::log_discord(
                &ctx.http,
                member.guild_id,
                &format!(
                    "Registered new member('{}') due to {} error",
                    member.user.name, why
                ),
            )
            .await;
        } else {
            sqlx::query("INSERT INTO unregistered_members VALUES (?, ?)")
                .bind(member.user.id.to_string())
                .bind(member.guild_id.to_string())
                .execute(connection)
                .await
                .expect("Query should be correct");
        }
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_some() {
            return;
        }

        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this moment");

        let command: Vec<&str> = message.content[crate::MAIN_BOT_PREFIX.len()..]
            .split_whitespace()
            .collect::<Vec<&str>>();
        let members: Vec<UnregisteredMember> = sqlx::query_as::<_, UnregisteredMember>(
            "SELECT * FROM unregistered_members WHERE discord_id = ?",
        )
        .bind(message.author.id.to_string())
        .fetch_all(connection)
        .await
        .expect("Query should be correct");
        match command[0] {
            "name" => {
                for member in members {
                    let member: Member = GuildId::from(member.guild_id)
                        .member(&ctx, UserId::from(member.discord_id))
                        .await
                        .expect("Member data should be correct");
                    let Setting { id: _, moderation_channel_id: _, log_channel_id: _, music_order_channel_id: _, music_log_channel_id: _, member_role_id } = sqlx::query_as::<_, Setting>("SELECT member_role_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)").bind(member.guild_id.to_string()).fetch_one(connection).await.expect("Query should be correct");
                    let member_role_id: RoleId = RoleId::from(Id(member_role_id
                        .0
                        .expect("member_role_id should be set at this moment")));
                    let member_roles: Vec<RoleId> = {
                        let mut vec: Vec<RoleId> = member.roles;
                        vec.push(member_role_id);
                        vec
                    };
                    let _ = member
                        .guild_id
                        .edit_member(&ctx.http, member.user.id, |m| {
                            m.roles(&member_roles)
                                .nickname(format!("{} <{}>", command[1], member.user.name))
                        })
                        .await;
                    sqlx::query("DELETE FROM unregistered_members WHERE discord_id = ?")
                        .bind(member.user.id.to_string())
                        .execute(connection)
                        .await
                        .expect("Query should be correct");
                    logger::log_discord(
                        &ctx.http,
                        member.guild_id,
                        &format!("Registered new member('{}')", member.user.name),
                    )
                    .await;
                }
            }
            _ => {}
        }
    }
}

async fn check_music_log_channel(guild_id: GuildId, channel_id: ChannelId) -> bool {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let Setting { id: _, moderation_channel_id: _, log_channel_id: _, music_order_channel_id: _, music_log_channel_id, member_role_id: _ } = sqlx::query_as::<_, Setting>("SELECT music_log_channel_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)").bind(guild_id.to_string()).fetch_one(connection).await.expect("Query should be correct");
    if let Some(music_log_channel_id) = music_log_channel_id.0 {
        if music_log_channel_id == channel_id.0 {
            return true;
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

    async fn guild_delete(&self, _: Context, incomplete: UnavailableGuild, _: Option<Guild>) {
        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this point");

        sqlx::query("DELETE FROM music_bots WHERE prefix = ? AND guild_id = ?")
        .bind(crate::MUSIC_BOT_PREFIXES[0].to_string())
        .bind(incomplete.id.to_string())
        .execute(connection)
        .await
        .expect("Query should be correct");
        logger::log(
            log::Level::Info,
            &format!("Unregistered MUSIC1_BOT from '{}' guild", incomplete.id),
        );
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_none()
            || !check_music_log_channel(
                message.guild_id.expect("First condition exclusives None"),
                message.channel_id,
            )
            .await
            || !message.content.starts_with(crate::MUSIC_BOT_PREFIXES[0])
        {
            return;
        }
        let command: Vec<&str> = message.content.split_whitespace().collect::<Vec<&str>>();
        match command[1] {
            "play" => play(&ctx, &message, MusicOrder::from(command[2..].join(" "))).await,
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

    async fn guild_delete(&self, _: Context, incomplete: UnavailableGuild, _: Option<Guild>) {
        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this point");

        sqlx::query("DELETE FROM music_bots WHERE prefix = ? AND guild_id = ?")
        .bind(crate::MUSIC_BOT_PREFIXES[1].to_string())
        .bind(incomplete.id.to_string())
        .execute(connection)
        .await
        .expect("Query should be correct");
        logger::log(
            log::Level::Info,
            &format!("Unregistered MUSIC1_BOT from '{}' guild", incomplete.id),
        );
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_none()
            || !check_music_log_channel(
                message.guild_id.expect("First condition exclusives None"),
                message.channel_id,
            )
            .await
            || !message.content.starts_with(crate::MUSIC_BOT_PREFIXES[1])
        {
            return;
        }
        let command: Vec<&str> = message.content.split_whitespace().collect::<Vec<&str>>();
        match command[1] {
            "play" => play(&ctx, &message, MusicOrder::from(command[2..].join(" "))).await,
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

    async fn guild_delete(&self, _: Context, incomplete: UnavailableGuild, _: Option<Guild>) {
        let connection: &SqlitePool = SERVERS_DB
            .get()
            .expect("Connection should be established at this point");

        sqlx::query("DELETE FROM music_bots WHERE prefix = ? AND guild_id = ?")
        .bind(crate::MUSIC_BOT_PREFIXES[2].to_string())
        .bind(incomplete.id.to_string())
        .execute(connection)
        .await
        .expect("Query should be correct");
        logger::log(
            log::Level::Info,
            &format!("Unregistered MUSIC1_BOT from '{}' guild", incomplete.id),
        );
    }

    async fn message(&self, ctx: Context, message: Message) {
        if message.guild_id.is_none()
            || !check_music_log_channel(
                message.guild_id.expect("First condition exclusives None"),
                message.channel_id,
            )
            .await
            || !message.content.starts_with(crate::MUSIC_BOT_PREFIXES[2])
        {
            return;
        }
        let command: Vec<&str> = message.content.split_whitespace().collect::<Vec<&str>>();
        match command[1] {
            "play" => play(&ctx, &message, MusicOrder::from(command[2..].join(" "))).await,
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
