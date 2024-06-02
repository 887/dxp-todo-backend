#[cfg(feature = "hot-reload")]
use tokio::sync::{mpsc::Receiver, RwLock};

#[cfg(feature = "hot-reload")]
use std::sync::Arc;

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
        hot_server::run_server().map_err(|e| format!("server aborted with error, {:?}", e))
    })
    .await??)
}

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
#[cfg(feature = "hot-reload")]
pub(crate) async fn run(
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>,
) {
    if let Err(err) = run_inner(server_running_writer, rx_shutdown_server).await {
        error!("running main_task failed: {:?}", err);
        error!("waiting 3s..");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

#[cfg(feature = "hot-reload")]
async fn run_inner(
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>,
) -> Result<()> {
    hot_server::load_env()?;

    #[cfg(feature = "migration")]
    run_migrations().await?;

    *server_running_writer.write().await = true;
    run_server(rx_shutdown_server).await
}

#[cfg(feature = "hot-reload")]
async fn run_server(rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<()> {
    // use std::{thread};
    // thread::spawn(|| {
    // }).join() {

    // https://stackoverflow.com/a/62536772
    // the tokio threadpool is used here
    Ok(tokio::task::spawn_blocking(|| {
        hot_server::run_server(rx_shutdown_server)
            .map_err(|e| format!("migration aborted with error, {:?}", e))
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
