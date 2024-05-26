use anyhow::Context;
#[cfg(all(debug_assertions, feature = "hot-reload"))]
use tokio::sync::{mpsc::Receiver, RwLock};

// use std::{thread};
#[cfg(all(debug_assertions, feature = "hot-reload"))]
use std::sync::Arc;

use crate::hot_libs::*;

#[cfg(any(not(debug_assertions), not(feature = "hot-reload")))]
pub(crate) async fn run() {
    if let Err(err) = run_inner().await {
        println!("running main_task failed: {:?}", err);
    }
}

#[cfg(any(not(debug_assertions), not(feature = "hot-reload")))]
async fn run_inner() -> Result<(), anyhow::Error> { 
    hot_lib::load_env()?;

    #[cfg(feature = "migration")]
    run_migrations().await?;

    match tokio::task::spawn_blocking(|| {
        hot_lib::run_server()
    // }).join() {
    }).await {
        Ok(res) => res,
        Err(err) => {
            return Err(err).context("run_server thread panicked");
        }
    }
}

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
#[cfg(all(debug_assertions, feature = "hot-reload"))]
pub(crate) async fn run (
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>) {
    if let Err(err) = run_inner(server_running_writer, rx_shutdown_server).await {
        println!("running main_task failed: {:?}", err);
        println!("waiting 3s..");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

#[cfg(all(debug_assertions, feature = "hot-reload"))]
async fn run_inner (
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {

    hot_lib::load_env()?;

    #[cfg(feature = "migration")]
    run_migrations().await?;

    *server_running_writer.write().await = true;
    run_server(rx_shutdown_server).await
}

#[cfg(all(debug_assertions, feature = "hot-reload"))]
async fn run_server(rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {
    // match thread::spawn(|| {
    match tokio::task::spawn_blocking(|| {
        hot_lib::run_server(rx_shutdown_server)
    // }).join() {
    }).await {
        Ok(res) => res,
        Err(err) => {
            Err(err).context("run_server thread panicked")
        }
    }
}

#[cfg(feature = "migration")]
async fn run_migrations() -> Result<(), anyhow::Error> {
    let run_migration_result = tokio::task::spawn_blocking(|| {
    hot_migration_runner::run_migration()
    // }).join() {
    }).await.context("run_migration thread panicked")?;
    run_migration_result.context("run migration failed")?;
    Ok(())
}