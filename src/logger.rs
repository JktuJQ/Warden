//! `log` module initializes and configures logger for this application.
//!

pub use log::Level;
use log::{log, LevelFilter};

use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

/// Initializes and configures logger.
///
pub fn init_logger() {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} {t} - {m}{n}")))
        .build(dotenv::var("LOG_FILE").expect("LOG_FILE should be provided"))
        .expect("Logfile should be created correctly");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .expect("Config should be created correctly");

    let _ = log4rs::init_config(config).expect("Logger should be initalized correctly");
}

/// Logs and prints message in stdout.
///  
pub fn log(level: Level, message: &str) {
    log!(level, "{}", message);
    println!("{}", message);
}

/// Logs in discord channel.
///
pub async fn log_discord(
    http: impl AsRef<serenity::http::Http>,
    guild_id: serenity::model::id::GuildId,
    message: &str,
) {
    let connection: &sqlx::SqlitePool = crate::db::connections::SERVERS_DB
        .get()
        .expect("Connection should be established at this moment");

    let log_channel_id: crate::db::models::ForeignId = sqlx::query_as::<_, crate::db::models::Setting>(
        "
        SELECT log_channel_id FROM settings WHERE id = (SELECT settings_id FROM guilds WHERE discord_id = ?)
    ",
    )
    .bind(guild_id.to_string())
    .fetch_one(connection)
    .await
    .expect("Query should be correct")
    .log_channel_id;

    if let Some(log_channel_id) = log_channel_id.0 {
        let log_channel_id: serenity::model::id::ChannelId =
            serenity::model::id::ChannelId(log_channel_id);
        if let Err(error) = log_channel_id.say(http, message).await {
            log(
                log::Level::Info,
                &format!("An error occured while trying to log in channel: {}", error),
            );
        } else {
            log(log::Level::Info, message);
        }
    }
}
