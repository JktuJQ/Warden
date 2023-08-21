use crate::logger;
use serenity::{
    client::Context,
    model::{channel::Message, guild::Guild, id::ChannelId},
};
use std::sync::Arc;

pub enum MusicOrder {
    Url(String),
    Query(String),
}
impl From<String> for MusicOrder {
    fn from(value: String) -> Self {
        if value.starts_with("https://") {
            MusicOrder::Url(value)
        } else {
            MusicOrder::Query(value)
        }
    }
}

pub async fn play(ctx: &Context, message: &Message, order: MusicOrder) {
    let guild: Guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild.id) {
        let mut handler = handler_lock.lock().await;

        let source = match match order {
            MusicOrder::Url(url) => songbird::input::ytdl(url).await,
            MusicOrder::Query(query) => songbird::input::ytdl_search(query).await,
        } {
            Ok(source) => source,
            Err(why) => {
                logger::log(log::Level::Info, &format!("Error sourcing ffmpeg: {}", why));
                return;
            }
        };
        handler.enqueue_source(source);
    }
}

pub async fn join(ctx: &Context, message: &Message, voice_channel_id: ChannelId) {
    let guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    let _ = manager.join(guild.id, voice_channel_id).await;
}
pub async fn leave(ctx: &Context, message: &Message) {
    let guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild.id).is_some();
    if has_handler {
        let _ = manager.remove(guild.id).await;
    }
}

pub async fn pause(ctx: &Context, message: &Message) {
    let guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild.id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.pause();
    }
}
pub async fn resume(ctx: &Context, message: &Message) {
    let guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild.id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.resume();
    }
}
pub async fn skip(ctx: &Context, message: &Message) {
    let guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild.id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        let _ = queue.skip();
    }
}
pub async fn stop(ctx: &Context, message: &Message) {
    let guild = message
        .guild(&ctx.cache)
        .expect("This will be called only from guilds");

    let manager: Arc<songbird::Songbird> = songbird::get(ctx)
        .await
        .expect("Songbird voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild.id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        queue.stop();
    }
}
