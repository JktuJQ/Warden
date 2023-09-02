//! `music_order` module implements commands that are used to order music and control corresponding music bots.
//!

use crate::{
    db::{
        connections::SERVERS_DB,
        models::{MusicBot, Setting},
    },
    logger,
};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{check, command, group},
        {Args, CommandOptions, CommandResult, Reason},
    },
    model::{
        channel::Message,
        id::{ChannelId, GuildId},
    },
};
use sqlx::SqlitePool;

/// `MusicOrder` struct is a group for `serenity` framework that contains music order commands.
///
#[group]
#[only_in(guilds)]
#[checks(music_order_channel)]
#[commands(play, join, leave, pause, resume, skip, stop)]
pub struct MusicOrder;

#[check]
#[name = "music_order_channel"]
async fn check_music_order_channel(
    _: &Context,
    message: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let Setting { music_order_channel_id, .. } = sqlx::query_as::<_, Setting>("SELECT music_order_channel_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)").bind(message.guild_id.expect("This should be called only on guilds").to_string()).fetch_one(connection).await.expect("Query should be correct");
    if let Some(channel_id) = music_order_channel_id.0 {
        if channel_id == message.channel_id.0 {
            return Ok(());
        }
    }
    Err(Reason::User(String::from("Wrong channel was used")))
}

async fn get_music_log_channel(guild_id: GuildId) -> Option<ChannelId> {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let Setting { music_log_channel_id, .. } = sqlx::query_as::<_, Setting>("SELECT music_log_channel_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)").bind(guild_id.to_string()).fetch_one(connection).await.expect("Query should be correct");
    music_log_channel_id.0.map(ChannelId)
}

#[command]
#[min_args(1)]
pub async fn play(ctx: &Context, message: &Message, args: Args) -> CommandResult {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let guild_id: GuildId = message
        .guild_id
        .expect("This should be called only on guilds");
    let voice_channel_id: Option<ChannelId> = message
        .guild(&ctx.cache)
        .expect("This should be called only on guilds")
        .voice_states
        .get(&message.author.id)
        .and_then(|voice_state| voice_state.channel_id);
    if voice_channel_id.is_none() {
        return Ok(());
    }
    let voice_channel_id: ChannelId = voice_channel_id.expect("None case was handled");

    let order: &str = args.remains().expect("At least one argument is supplied");
    let music_bot: Option<MusicBot> = sqlx::query_as::<_, MusicBot>(
        "SELECT prefix FROM music_bots WHERE guild_id = ? AND on_channel_id = ?",
    )
    .bind(guild_id.to_string())
    .bind(voice_channel_id.to_string())
    .fetch_optional(connection)
    .await
    .expect("Query should be correct");
    if let Some(MusicBot {
        prefix,
        ..
    }) = music_bot
    {
        if let Some(channel_id) = get_music_log_channel(guild_id).await {
            channel_id
                .say(&ctx.http, format!("{}play {}", prefix, order))
                .await?;
            message
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "➡️ 🎵 Playing 🎶{}🎶 song on '{}' voice channel!!! ⬅️",
                        order,
                        voice_channel_id
                            .to_channel(&ctx.http)
                            .await
                            .expect("Channel exists on this guild")
                            .guild()
                            .expect("This channel is on guild")
                            .name
                    ),
                )
                .await?;
            logger::log_discord(&ctx.http, guild_id, &format!("Called play on {}", prefix)).await;
        }
    }

    Ok(())
}

#[command]
#[num_args(0)]
pub async fn join(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let guild_id: GuildId = message
        .guild_id
        .expect("This should be called only on guilds");
    let voice_channel_id: Option<ChannelId> = message
        .guild(&ctx.cache)
        .expect("This should be called only on guilds")
        .voice_states
        .get(&message.author.id)
        .and_then(|voice_state| voice_state.channel_id);
    if voice_channel_id.is_none() {
        return Ok(());
    }
    let voice_channel_id: ChannelId = voice_channel_id.expect("None case was handled");

    if sqlx::query("SELECT * FROM music_bots WHERE guild_id = ? AND on_channel_id = ?")
        .bind(guild_id.to_string())
        .bind(voice_channel_id.to_string())
        .fetch_optional(connection)
        .await
        .expect("Query should be correct")
        .is_some()
    {
        return Ok(());
    }
    let music_bot: Option<MusicBot> = sqlx::query_as::<_, MusicBot>(
        "SELECT prefix FROM music_bots WHERE guild_id = ? AND on_channel_id IS NULL",
    )
    .bind(guild_id.to_string())
    .fetch_optional(connection)
    .await
    .expect("Query should be correct");
    if let Some(MusicBot {
        prefix,
        ..
    }) = music_bot
    {
        if let Some(channel_id) = get_music_log_channel(guild_id).await {
            channel_id
                .say(&ctx.http, format!("{}join {}", prefix, voice_channel_id))
                .await?;
            message
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "➡️ 👍 Joined '{}' voice channel!!! ⬅️",
                        voice_channel_id
                            .to_channel(&ctx.http)
                            .await
                            .expect("Channel exists on this guild")
                            .guild()
                            .expect("This channel is on guild")
                            .name
                    ),
                )
                .await?;
            sqlx::query(
                "
                INSERT INTO channels VALUES (?, ?);
                UPDATE music_bots SET on_channel_id = ? WHERE guild_id = ? AND prefix = ?;
            ",
            )
            .bind(voice_channel_id.to_string())
            .bind(guild_id.to_string())
            .bind(voice_channel_id.to_string())
            .bind(guild_id.to_string())
            .bind(prefix.clone())
            .execute(connection)
            .await
            .expect("Query should be correct");
            logger::log_discord(&ctx.http, guild_id, &format!("Called join on {}", prefix)).await;
        }
    }

    Ok(())
}
#[command]
#[num_args(0)]
pub async fn leave(ctx: &Context, message: &Message, _: Args) -> CommandResult {
    let connection: &SqlitePool = SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let guild_id: GuildId = message
        .guild_id
        .expect("This should be called only on guilds");
    let voice_channel_id: Option<ChannelId> = message
        .guild(&ctx.cache)
        .expect("This should be called only on guilds")
        .voice_states
        .get(&message.author.id)
        .and_then(|voice_state| voice_state.channel_id);
    if voice_channel_id.is_none() {
        return Ok(());
    }
    let voice_channel_id: ChannelId = voice_channel_id.expect("None case was handled");

    let music_bot: Option<MusicBot> = sqlx::query_as::<_, MusicBot>(
        "SELECT prefix FROM music_bots WHERE guild_id = ? AND on_channel_id = ?",
    )
    .bind(guild_id.to_string())
    .bind(voice_channel_id.to_string())
    .fetch_optional(connection)
    .await
    .expect("Query should be correct");

    if let Some(MusicBot {
        prefix,
        ..
    }) = music_bot
    {
        if let Some(channel_id) = get_music_log_channel(guild_id).await {
            channel_id
                .say(&ctx.http, format!("{}leave {}", prefix, voice_channel_id))
                .await?;
            message
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "➡️ 😔 Left '{}' voice channel :( ⬅️",
                        voice_channel_id
                            .to_channel(&ctx.http)
                            .await
                            .expect("Channel exists on this guild")
                            .guild()
                            .expect("This channel is on guild")
                            .name
                    ),
                )
                .await?;
            sqlx::query("DELETE FROM channels WHERE discord_id = ?")
                .bind(voice_channel_id.to_string())
                .execute(connection)
                .await
                .expect("Query should be correct");
            logger::log_discord(&ctx.http, guild_id, &format!("Called leave on {}", prefix)).await;
        }
    }

    Ok(())
}

macro_rules! no_args_command {
    ($fullname:ident) => {
        #[command]
        #[num_args(0)]
        pub async fn $fullname(ctx: &Context, message: &Message, _: Args) -> CommandResult {
            let connection: &SqlitePool = SERVERS_DB
                .get()
                .expect("Connection should be established at this moment");

            let guild_id: GuildId = message
                .guild_id
                .expect("This should be called only on guilds");
            let voice_channel_id: Option<ChannelId> = message
                .guild(&ctx.cache)
                .expect("This should be called only on guilds")
                .voice_states
                .get(&message.author.id)
                .and_then(|voice_state| voice_state.channel_id);
            if voice_channel_id.is_none() {
                return Ok(());
            }

            let music_bot: Option<MusicBot> = sqlx::query_as::<_, MusicBot>(
                "SELECT prefix FROM music_bots WHERE guild_id = ? AND on_channel_id = ?",
            )
            .bind(guild_id.to_string())
            .bind(voice_channel_id.expect("None case was handled").to_string())
            .fetch_optional(connection)
            .await
            .expect("Query should be correct");

            if let Some(MusicBot {
                prefix,
                ..
            }) = music_bot
            {
                if let Some(channel_id) = get_music_log_channel(guild_id).await {
                    channel_id
                        .say(
                            &ctx.http,
                            format!(concat!("{}", stringify!($fullname)), prefix),
                        )
                        .await?;
                    message
                        .channel_id
                        .say(
                            &ctx.http,
                            concat!("➡️ Called ", stringify!($fullname), " on current queue!!! ⬅️"),
                        )
                        .await?;
                    logger::log_discord(
                        &ctx.http,
                        guild_id,
                        &format!(concat!("Called ", stringify!($fullname), " on {}"), prefix),
                    )
                    .await;
                }
            }

            Ok(())
        }
    };
}
no_args_command!(pause);
no_args_command!(resume);
no_args_command!(skip);
no_args_command!(stop);
