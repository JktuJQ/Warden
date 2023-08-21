//! # Warden
//!
//! **Warden** is a multifunctional discord bot that handles registrations, music, entertainment and more.
//!

// modules
mod logger;
use crate::logger::init_logger;

mod db;
use crate::db::init_db;

mod bot;
use crate::bot::Bot;

const MAIN_BOT_PREFIX: &str = "-";
const MUSIC_BOT_PREFIXES: [&str; 3] = ["music1 ", "music2 ", "music3 "];

#[tokio::main]
async fn main() {
    let _ = dotenv::from_filename("SETTINGS.env").ok();

    init_logger();
    init_db().await;

    let main_token: String = dotenv::var("MAIN_DISCORD_TOKEN").expect("Token should be provided");
    let music1_token: String =
        dotenv::var("MUSIC1_DISCORD_TOKEN").expect("Token should be provided");
    let music2_token: String =
        dotenv::var("MUSIC2_DISCORD_TOKEN").expect("Token should be provided");
    let music3_token: String =
        dotenv::var("MUSIC3_DISCORD_TOKEN").expect("Token should be provided");

    let mut main_bot: Bot = Bot::new(
        main_token,
        MAIN_BOT_PREFIX,
        vec![
            &bot::GENERAL_GROUP,
            &bot::settings::SETTINGS_GROUP,
            &bot::music_order::MUSICORDER_GROUP,
        ],
        bot::events::Handler,
        false,
    )
    .await;
    let mut music1_bot: Bot = Bot::new(
        music1_token,
        MUSIC_BOT_PREFIXES[0],
        vec![],
        bot::events::MusicHandler1,
        true,
    )
    .await;
    let mut music2_bot: Bot = Bot::new(
        music2_token,
        MUSIC_BOT_PREFIXES[1],
        vec![],
        bot::events::MusicHandler2,
        true,
    )
    .await;
    let mut music3_bot: Bot = Bot::new(
        music3_token,
        MUSIC_BOT_PREFIXES[2],
        vec![],
        bot::events::MusicHandler3,
        true,
    )
    .await;
    tokio::spawn(async move {
        if let Err(error) = main_bot.run().await {
            logger::log(
                log::Level::Error,
                &format!("An error occurred while running the main_bot: {:?}", error),
            );
        };
    });
    tokio::spawn(async move {
        if let Err(error) = music1_bot.run().await {
            logger::log(
                log::Level::Error,
                &format!(
                    "An error occurred while running the music2_bot: {:?}",
                    error
                ),
            );
        };
    });
    tokio::spawn(async move {
        if let Err(error) = music2_bot.run().await {
            logger::log(
                log::Level::Error,
                &format!(
                    "An error occurred while running the music2_bot: {:?}",
                    error
                ),
            );
        };
    });
    tokio::spawn(async move {
        if let Err(error) = music3_bot.run().await {
            logger::log(
                log::Level::Error,
                &format!(
                    "An error occurred while running the music3_bot: {:?}",
                    error
                ),
            );
        };
    });
    let _ = tokio::signal::ctrl_c().await;
}
