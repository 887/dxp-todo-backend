use tracing::error;

use super::get_log_subscription;

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    dotenvy::dotenv()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not load .env"))?;

    #[cfg(feature = "log")]
    let log_subscription = get_log_subscription()?;
    let res = run().await;
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}

pub(crate) async fn run() -> std::io::Result<()> {
    if let Err(err) = run_inner().await {
        error!("running main_task failed: {:?}", err);
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ));
    }
    Ok(())
}

async fn run_inner() -> crate::Result<()> {
    server::load_env()?;

    #[cfg(feature = "migration")]
    run_migrations().await?;

    Ok(tokio::task::spawn_blocking(|| {
        server::run_server().map_err(|e| format!("run_server aborted with error: {:?}", e))
    })
    .await??)
}

#[cfg(feature = "migration")]
pub async fn run_migrations() -> crate::Result<()> {
    Ok(tokio::task::spawn_blocking(|| {
        migration_runner::run_migration()
            .map_err(|e| format!("migration aborted with error, {:?}", e))
    })
    .await??)
}
