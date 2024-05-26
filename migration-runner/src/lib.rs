#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

use sea_orm::{ DatabaseConnection, DbErr};

// pub fn run_migration(rt: tokio::runtime::Handle, db: DatabaseConnection) -> Result<(), anyhow::Error> {
//     rt.block_on(async {
//         run_migrator(db).await?;
//         Ok(())
//     })
// }

pub async fn run_migrator(db: &DatabaseConnection) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    Migrator::up(db, None).await?;

    Ok(())
}


