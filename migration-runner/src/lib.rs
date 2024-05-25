use sea_orm::{ DatabaseConnection, DbErr};

pub fn run_migration(rt: tokio::runtime::Handle, db: DatabaseConnection) -> Result<(), anyhow::Error> {
    rt.block_on(async {
        run_migrator(db).await?;
        Ok(())
    })
}

async fn run_migrator(db: DatabaseConnection) -> Result<(), DbErr> {
    use migration::{Migrator, MigratorTrait};

    println!("updating db");
    Migrator::up(&db, None).await?;

    Ok(())
}

