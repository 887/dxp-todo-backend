use migration::sea_orm::Database;
use sea_orm::DbErr;

#[cfg(debug_assertions)]
#[no_mangle]
pub async fn run_migration(db_url: &str) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    let db = Database::connect(db_url).await?;

    Ok(Migrator::up(&db, None).await?)
}
