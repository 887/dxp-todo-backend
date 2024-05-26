#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

// use std::{thread};
use std::sync::{Arc};

use tokio::sync::mpsc::{Receiver};
use tokio::sync::{Mutex, RwLock};
use tokio::{sync::mpsc};

 mod observe_reloads;

//tokio hot reload example
//https://github.com/rksm/hot-lib-reloader-rs/blob/master/examples/reload-events/src/main.rs

#[hot_lib_reloader::hot_module(dylib = "lib")]
mod hot_lib {
    // pub use lib::*;

    hot_functions_from_file!("lib/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[hot_lib_reloader::hot_module(dylib = "migration_runner")]
mod hot_migration_runner {
    // pub use lib::*;

    hot_functions_from_file!("migration-runner/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Use RUST_LOG=hot_lib_reloader=trace to see all related logs
    env_logger::init();

    #[cfg(feature = "path-info")]
    print_paths();

    //this channel is to shut down the server 
    let (tx_shutdown_server, rx_shutdown_server) = mpsc::channel(1);
    let rx_shutdown_server = Arc::new(RwLock::new(rx_shutdown_server));

    //ensures that the server and reloads are blocking
    let block_reloads_mutex = Arc::new(Mutex::new(0));
    let block_reloads_mutex_main = block_reloads_mutex.clone();

    //this is mainly so we don't send messages to a dead server 
    let server_is_running = Arc::new(RwLock::new(false));
    let server_is_running_writer = server_is_running.clone();
    let server_is_running_reader = server_is_running.clone();

    tokio::task::spawn(async move {
        observe_reloads::run_observe(
            server_is_running_reader,
            tx_shutdown_server,
            block_reloads_mutex).await
    });

    //main loop
    loop {
        //only run when we can access the mutex
        let lock = block_reloads_mutex_main.lock().await;

        println!("------------main loop------------");

        run_main_task(server_is_running_writer.clone(), rx_shutdown_server.clone()).await;

        println!("------------main loop finished------------");

        //only allow more reloads when we are finished
        drop(lock);
    }
}

//everything that can fail needs to be in this task
//once this task finishes the hot-reload-lib checks if there is a new library to reload
async fn run_main_task (
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

fn run_migration() -> Result<(), anyhow::Error> {
    hot_migration_runner::run_migration()
}

fn run_server(rx_shutdown_server: Arc<RwLock<Receiver<()>>>) -> Result<(), anyhow::Error> {
    hot_lib::run_server(rx_shutdown_server)
}

#[cfg(feature = "path-info")]
fn get_current_working_dir() -> String {
    let res = std::env::current_dir();
    match res {
        Ok(path) => path.into_os_string().into_string().unwrap(),
        Err(_) => "FAILED".to_string(),
    }
}

#[cfg(feature = "path-info")]
fn get_lib_path() -> String {
    let res = std::env::var("LD_LIBRARY_PATH");
    match res {
        Ok(path) => path.to_string(),
        Err(_) => "FAILED".to_string(),
    }
}

#[cfg(feature = "path-info")]
fn print_paths() {
    println!("working dir: {}", get_current_working_dir());
    println!("lib path: {}", get_lib_path());
}