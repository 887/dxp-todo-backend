use anyhow::Context;
use tokio::sync::{mpsc::Receiver, RwLock};

// use std::{thread};
use std::sync::Arc;

use crate::hot_libs::*;

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
pub(crate) async fn run (
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>) {
    match run_inner(server_running_writer, rx_shutdown_server).await {
        Ok(_) => {},
        Err(err) => {
            println!("running main_task failed: {:?}", err);
            println!("waiting 3s..");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        },
    }
}

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

pub(crate) fn run_migration() -> Result<(), anyhow::Error> {
    hot_migration_runner::run_migration()
}

pub(crate) fn run_server(rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {
    hot_lib::run_server(rx_shutdown_server)
}
