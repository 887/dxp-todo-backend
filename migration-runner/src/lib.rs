use anyhow::Context;
use migration::sea_orm::Database;
use sea_orm::DbErr;
use tokio::runtime::Runtime;
use std::sync::Arc;
use anyhow::anyhow;

pub fn run_migration(db_url: &str) -> Box<Result<tokio::task::JoinHandle<()>, anyhow::Error>> {
    //a new runtime is needed here, since the original one is busy with the hot-reload
    let rt  = match Runtime::new() {
        Ok(rt) => rt,
        Err(err) => return Box::new(Err(anyhow!("failed to create runtime {}", err)))
    };

    let db_url_heap = db_url.to_string();
    let jh = rt.spawn(async {
        println!("running migrator");
        run_migrator(db_url_heap).await;
        println!("migration done");
    });
    Box::new(Ok(jh))
}


async fn run_migrator(db_url: String) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    let db = Database::connect(db_url).await?;

    Ok(Migrator::up(&db, None).await?)
}

