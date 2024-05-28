use anyhow::Context;
use sea_orm::{DatabaseConnection, DbErr};

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_migration_main() -> Result<(), anyhow::Error> {
    println!("running migration");

    let db = dbopen::get_database_connection()
        .await
        .context("could not get db connection")?;

    let result = match run_migrator(&db).await {
        Ok(_) => Ok(()),
        Err(err) => Err(anyhow::anyhow!("migration failed: {}", err)),
    };

    //ensure we always close the database here
    db.close().await?;

    return result;
}

pub async fn run_migrator(db: &DatabaseConnection) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    Migrator::up(db, None).await?;

    Ok(())
}
