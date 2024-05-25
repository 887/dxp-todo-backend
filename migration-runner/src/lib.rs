use migration::sea_orm::Database;
use sea_orm::DbErr;
use tokio::runtime::Runtime;
use std::sync::Arc;

pub fn run_migration(rt: Arc<Box<tokio::runtime::Handle>>, db_url: &str) -> Box<tokio::task::JoinHandle<()>> {
    // print!("running migrator");

    let rt  = Runtime::new().unwrap();

    let db_url_heap = db_url.to_string();
    let bx = rt.spawn(async {
        println!("running migrator");
        run_migrator(db_url_heap).await;
        println!("migration done");
    });
    Box::new(bx)
}

async fn run_migrator(db_url: String) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    let db = Database::connect(db_url).await?;

    Ok(Migrator::up(&db, None).await?)
}

