#[cfg(feature = "hot-reload")]
use tokio::sync::mpsc::Receiver;

use tracing::error;

use crate::hot_libs::*;

#[cfg(not(feature = "hot-reload"))]
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

#[cfg(not(feature = "hot-reload"))]
async fn run_inner() -> Result<()> {
    hot_server::load_env()?;

    #[cfg(feature = "migration")]
    run_migrations().await?;

    Ok(tokio::task::spawn_blocking(|| {
        hot_server::run_server().map_err(|e| format!("run_server aborted with error: {:?}", e))
    })
    .await??)
}

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
#[cfg(feature = "hot-reload")]
pub(crate) async fn run(rx_shutdown_server: Receiver<()>) {
    if let Err(err) = run_inner(rx_shutdown_server).await {
        error!("running main_task failed: {:?}", err);
        error!("waiting 3s..");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

#[cfg(feature = "hot-reload")]
async fn run_inner(rx_shutdown_server: Receiver<()>) -> Result<()> {
    hot_server::load_env()?;

    #[cfg(feature = "migration")]
    run_migrations().await?;

    run_server(rx_shutdown_server).await
}

#[cfg(feature = "hot-reload")]
async fn run_server(rx_shutdown_server: Receiver<()>) -> Result<()> {
    // use std::{thread};
    // thread::spawn(|| {
    // }).join() {

    // https://stackoverflow.com/a/62536772
    // the tokio threadpool is used here

    Ok(tokio::task::spawn_blocking(|| {
        hot_server::run_server(rx_shutdown_server)
            .map_err(|e| format!("run_server aborted with error: {:?}", e))
    })
    .await??)
}

#[cfg(feature = "migration")]
async fn run_migrations() -> Result<()> {
    Ok(tokio::task::spawn_blocking(|| {
        hot_migration_runner::run_migration()
            .map_err(|e| format!("migration aborted with error, {:?}", e))
    })
    .await??)
}
