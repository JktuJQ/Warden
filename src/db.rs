//! `db` module implements database interaction interface.
//!

pub mod connections;
pub mod models;

/// Initializes databases.
///
pub async fn init_db() {
    connections::SERVERS_DB
        .set(
            connections::connect(
                &dotenv::var("SERVERS_DATABASE_FILE")
                    .expect("SERVERS_DATABASE_FILE should be provided"),
            )
            .await,
        )
        .expect("It should be possible to connect to SERVERS_DB");
}
