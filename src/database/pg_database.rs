use std::env;
use std::time::Duration;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use crate::config::Config;

pub type PgPooledConnection = sqlx::pool::PoolConnection<Postgres>;
pub type PgPool = Pool<Postgres>;

pub async fn connect(cfg: &Config) -> Result<PgPool, sqlx::Error> {
    log::info!("Connecting to database: {}", cfg.database_url);

    let pool = PgPoolOptions::new()
        .min_connections(cfg.database_min_connections)
        .max_connections(cfg.database_max_connections)
        .max_lifetime(Some(Duration::from_secs(cfg.database_max_lifetime)))
        .connect(&cfg.database_url)
        .await?;


    Ok(pool)
}

// pub async fn check_for_migrations(database_url: &str) -> Result<(), sqlx::Error> {
//     if !Postgres::database_exists(database_url).await? {
//         log::info!("Database does not exist, creating it...");
//         Postgres::create_database(database_url).await?;
//     }
//
//     log::info!("Running migrations...");
//
//     let mut conn: PgConnection = PgConnection::connect(database_url).await?;
//
//     sqlx::migrate!()
//         .run(&mut conn)
//         .await
//         .expect("Failed to run migrations");
//
//     Ok(())
// }