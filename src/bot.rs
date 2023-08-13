//! `bot` module contains all bot functions, handlers, etc.
//!

// submodules
pub mod events;
pub mod settings;

use serenity::{
    client::{Client, ClientBuilder},
    framework::standard::{
        macros::group,
        {CommandGroup, StandardFramework},
    },
    model::{gateway::GatewayIntents, id::UserId},
    prelude::EventHandler,
    Result,
};
use songbird::SerenityInit;

/// `General` struct is a global group for `serenity` framework.
///
#[group]
pub struct General;

/// `Bot` struct represents discord bot.
///
pub struct Bot {
    /// Underlying client.
    ///
    client: Client,
}
impl Bot {
    /// Prefix of bot.
    /// 
    pub const PREFIX: &str = "-";
    /// Array of bot developers (owners).
    ///
    pub const OWNERS: [UserId; 1] = [UserId(290146364346597376)];

    /// Initializes new bot with given token.
    ///
    pub async fn new(
        token: String,
        groups: Vec<&'static CommandGroup>,
        event_handler: impl EventHandler + 'static,
        is_music: bool,
    ) -> Self {
        let mut framework: StandardFramework =
            StandardFramework::new().configure(|c| c.prefix(Self::PREFIX).owners(Self::OWNERS.into()));
        for group in groups {
            framework = framework.group(group);
        }

        let intents: GatewayIntents = GatewayIntents::all();

        let client: Client = {
            let mut client: ClientBuilder = Client::builder(token, intents)
                .event_handler(event_handler)
                .framework(framework);
            if is_music {
                client = client.register_songbird();
            }
            client.await.expect("Client should be created correctly")
        };

        Bot { client }
    }

    /// Runs bot.
    ///
    pub async fn run(&mut self) -> Result<()> {
        self.client.start().await
    }
}
