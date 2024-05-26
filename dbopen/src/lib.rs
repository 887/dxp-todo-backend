#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use std::env;
use anyhow::Context;

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn get_database_connection() -> Result<DatabaseConnection, anyhow::Error> {
    let db_url = get_database_url()?;

    // println!("connecting db");
    let mut opt = ConnectOptions::new(db_url);
    opt.max_connections(100).min_connections(0);
    let db = Database::connect(opt).await?;
    Ok(db)
}

fn get_database_url() -> Result<String, anyhow::Error> {
    env::var("DATABASE_URL").context("DATABASE_URL is not set in .env file")
}
