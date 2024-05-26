use tokio::sync::{mpsc::Receiver, RwLock};

use std::sync::Arc;

use crate::hot_libs::*;

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
pub(crate) async fn run (
    server_running_writer: Arc<RwLock<bool>>,
    rx_shutdown_server: Arc<RwLock<Receiver<()>>>) {

    let wait = async move {
        println!("trying again in 3s");
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    };

    if let Err(load_err) = hot_lib::load_env() {
        println!("hot_lib::load_env: {}", load_err);
        wait.await;
        return
    }

    //using threads here causes panics, because the runtime for the migration is also tokio, so we use tokio tasks
    //also important read:
    //https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
    // let migration_result = match thread::spawn(|| {
    let run_migration_result = match tokio::task::spawn_blocking(|| {
        run_migration()
    // }).join() {
    }).await {
        Ok(res) => res,
        Err(_err) => {
            println!("run migration thread panicked");
            wait.await;
            return;
        }
    };

    if let Err(err) = run_migration_result {
        println!("run migration failed: {}", err);
        wait.await;
        return;
    }

    *server_running_writer.write().await = true;

    // match thread::spawn(|| {
    let run_server_result = match tokio::task::spawn_blocking(|| {
        run_server(rx_shutdown_server)
    // }).join() {
    }).await {
        Ok(res) => res,
        Err(_err) => {
            *server_running_writer.write().await = false;
            println!("run migration thread panicked");
            wait.await;
            return;
        }
    };

    if let Err(err) = run_server_result {
        println!("run server failed: {}", err);
        wait.await;
    }
}

pub(crate) fn run_migration() -> Result<(), anyhow::Error> {
    hot_migration_runner::run_migration()
}

pub(crate) fn run_server(rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {
    hot_lib::run_server(rx_shutdown_server)
}
