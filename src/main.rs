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

#[tokio::main]
async fn main() {
    let _ = dotenv::from_filename("SETTINGS.env").ok();

    init_logger();
    init_db().await;

    let main_token: String = dotenv::var("MAIN_DISCORD_TOKEN").expect("Token should be provided");

    let mut main_bot: Bot = Bot::new(
        main_token,
        vec![
            &bot::GENERAL_GROUP,
        ],
        bot::events::Handler,
        false,
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
    let _ = tokio::signal::ctrl_c().await;
}
