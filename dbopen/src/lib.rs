#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn Error>>;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env, error::Error};

pub async fn get_database_connection() -> Result<DatabaseConnection> {
    let db_url = get_database_url()?;

    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100).min_connections(0);
    let db = Database::connect(opt).await?;
    Ok(db)
}

fn get_database_url() -> Result<String> {
    Ok(env::var("DATABASE_URL").map_err(|_| "DATABASE_URL is not set")?)
}
