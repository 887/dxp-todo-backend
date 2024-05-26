use anyhow::Context;
#[cfg(debug_assertions)]
use tokio::sync::{mpsc::Receiver, RwLock};

// use std::{thread};
#[cfg(debug_assertions)]
use std::sync::Arc;

use crate::hot_libs::*;

#[cfg(not(debug_assertions))]
pub(crate) async fn run() {
    if let Err(err) = run_inner().await {
        println!("running main_task failed: {:?}", err);
    }
}

#[cfg(not(debug_assertions))]
async fn run_inner() -> Result<(), anyhow::Error> { 
    hot_lib::load_env()?;
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
#[cfg(debug_assertions)]
pub(crate) async fn run (
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>) {
    if let Err(err) = run_inner(server_running_writer, rx_shutdown_server).await {
        println!("running main_task failed: {:?}", err);
        println!("waiting 3s..");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

#[cfg(debug_assertions)]
async fn run_inner (
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {

    hot_lib::load_env()?;

    //using threads here causes panics, because the runtime for the migration is also tokio, so we use tokio tasks
    //also important read:
    //https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
    // let migration_result = match thread::spawn(|| {
    let run_migration_result = tokio::task::spawn_blocking(|| {
        run_migration()
    // }).join() {
    }).await.context("run_migration thread panicked")?;

    run_migration_result.context("run migration failed")?;

    *server_running_writer.write().await = true;

    // match thread::spawn(|| {
    match tokio::task::spawn_blocking(|| {
        run_server(rx_shutdown_server)
    // }).join() {
    }).await {
        Ok(res) => res,
        Err(err) => {
            *server_running_writer.write().await = false;
            return Err(err).context("run_server thread panicked");
        }
    }
}

#[cfg(debug_assertions)]
pub(crate) fn run_migration() -> Result<(), anyhow::Error> {
    hot_migration_runner::run_migration()
}

#[cfg(debug_assertions)]
pub(crate) fn run_server(rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {
    hot_lib::run_server(rx_shutdown_server)
}
