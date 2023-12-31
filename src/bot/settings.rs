//! `settings` module implements commands that are used to setup guild settings and
//! are requiring administrator permissions to be called.
//!

use crate::{
    db::{connections::SERVERS_DB, models::Id},
    logger,
};
use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        {Args, CommandResult},
    },
    model::{
        channel::Message,
        id::{ChannelId, GuildId, RoleId},
    },
};
use sqlx::SqlitePool;

/// `Settings` struct is a group for `serenity` framework that contains settings commands.
///
#[group]
#[required_permissions("ADMINISTRATOR")]
#[only_in(guilds)]
#[commands(
    set_log_channel,
    set_moderation_channel,
    set_music_order_channel,
    set_music_log_channel,
    set_member_role
)]
pub struct Settings;

macro_rules! setup_channel {
    ($fullname:ident, $name:ident) => {
        #[command]
        #[num_args(1)]
        pub async fn $fullname(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
            let connection: &SqlitePool = SERVERS_DB.get().expect("Connection should already be established at this moment");

            let guild_id: Id = message.guild_id.expect(concat!(stringify!($fullname), " command can be called only on guild")).into();
            let channel_id: Id = args.single::<Id>()?.into();
            if ChannelId::from(channel_id).to_channel(&ctx.http).await.is_err() {
                return Ok(());
            }
            if sqlx::query("SELECT discord_id FROM channels WHERE discord_id = ?")
                .bind(channel_id.to_string())
                .fetch_optional(connection)
                .await
                .expect("Query should be correct").is_some() {
                sqlx::query("DELETE FROM channels WHERE discord_id = ?")
                    .bind(channel_id.to_string())
                    .execute(connection)
                    .await
                    .expect("Query should be correct");
            }

            sqlx::query(concat!("
                INSERT INTO channels VALUES (?, ?);
                UPDATE settings SET ", stringify!($name), "_id = (SELECT last_insert_rowid()) WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)"
            )).bind(channel_id.to_string()).bind(guild_id.to_string())
            .bind(guild_id.to_string()).execute(connection).await.expect("Query should be correct");

            logger::log(log::Level::Info, concat!(stringify!($fullname), " was called"));
            Ok(())
        }
    };
}
setup_channel!(set_log_channel, log_channel);
setup_channel!(set_moderation_channel, moderation_channel);
setup_channel!(set_music_order_channel, music_order_channel);
setup_channel!(set_music_log_channel, music_log_channel);

macro_rules! setup_roles {
    ($fullname:ident, $name:ident) => {
        #[command]
        #[num_args(1)]
        pub async fn $fullname(ctx: &Context, message: &Message, mut args: Args) -> CommandResult {
            let connection: &SqlitePool = SERVERS_DB
                .get()
                .expect("Connection should already be established at this moment");

            let guild_id: Id = message
                .guild_id
                .expect(concat!(
                    stringify!($fullname),
                    " command can be called only on guild"
                ))
                .into();
            let role_id: Id = args.single::<Id>()?.into();
            if !GuildId::from(guild_id)
                .roles(&ctx.http)
                .await
                .expect(concat!(
                    stringify!($fullname),
                    " command can be called only on guild"
                ))
                .contains_key(&RoleId::from(role_id))
            {
                return Ok(());
            }
            if sqlx::query("SELECT discord_id FROM roles WHERE discord_id = ? AND guild_id = ?")
                .bind(role_id.to_string())
                .bind(guild_id.to_string())
                .fetch_optional(connection)
                .await
                .expect("Query should be correct")
                .is_some()
            {
                sqlx::query("DELETE FROM roles WHERE discord_id = ? AND guild_id = ?")
                    .bind(role_id.to_string())
                    .bind(guild_id.to_string())
                    .execute(connection)
                    .await
                    .expect("Query should be correct");
            }

            sqlx::query(concat!(
                "
                INSERT INTO roles VALUES (?, ?);
                UPDATE settings SET ",
                stringify!($name),
                "_id = ? WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)"
            ))
            .bind(role_id.to_string())
            .bind(guild_id.to_string())
            .bind(role_id.to_string())
            .bind(guild_id.to_string())
            .execute(connection)
            .await
            .expect("Query should be correct");

            logger::log(
                log::Level::Info,
                concat!(stringify!($fullname), " was called"),
            );
            Ok(())
        }
    };
}
setup_roles!(set_member_role, member_role);
