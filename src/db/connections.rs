//! `db` module provides `connect` function and several global static connection handlers for `Warden` bot.
//!

use crate::logger::log;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::sync::OnceLock;

/// Returns new connection pool to `sqlite` database.
///
pub async fn connect(name: &str) -> SqlitePool {
    let db_url: &str = &format!("sqlite://{}", name);

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        Sqlite::create_database(db_url)
            .await
            .expect("Creation of database should be possible");
        log(log::Level::Info, &format!("Created database {}", db_url));
    }
    SqlitePool::connect(db_url).await.unwrap()
}

/// Global `servers.db` pool.
///
pub static SERVERS_DB: OnceLock<SqlitePool> = OnceLock::new();
