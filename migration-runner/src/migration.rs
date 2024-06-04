use sea_orm::DatabaseConnection;
use tracing::info;

use crate::Result;

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_migration_main() -> Result<()> {
    info!("running migration");

    let db = dxp_db_open::get_database_connection().await?;

    run_migrator(&db).await?;

    //ensure we always close the database here
    db.close().await?;

    Ok(())
}

pub async fn run_migrator(db: &DatabaseConnection) -> Result<()> {
    use migration::{Migrator, MigratorTrait};

    Ok(Migrator::up(db, None)
        .await
        .map_err(|e| format!("Migration error: {:?}", e))?)
}
