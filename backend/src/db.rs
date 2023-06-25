use sqlx::{migrate, postgres::PgPoolOptions, Pool, Postgres};

use crate::config::{self, Config};

pub mod posts;
pub mod user_credentials;
pub mod users;

/// Creates the database pool from config and runs migrations
pub async fn setup_pool_from_config(config: &Config) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    if let config::Environment::PRODUCTION = &config.environment {
        migrate!("./migrations").run(&pool).await?;
    }

    return Ok(pool);
}
