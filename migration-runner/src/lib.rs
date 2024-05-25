use migration::sea_orm::Database;
use sea_orm::DbErr;
use tokio::runtime::Runtime;

pub fn run_migration(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // print!("running migrator");

    let rt  = Runtime::new()?;

    let db_url_heap = db_url.to_string();
    let res = rt.block_on(async {
        // println!("running async");
        run_migrator(db_url_heap).await
    });
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

async fn run_migrator(db_url: String) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    let db = Database::connect(db_url).await?;

    Ok(Migrator::up(&db, None).await?)
}

