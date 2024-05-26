#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

// use std::{thread};
use std::sync::{Arc};

use tokio::sync::{Mutex, RwLock};
use tokio::{sync::mpsc};

mod observe;
mod path_info;
mod hot_libs;

mod main_task;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Use RUST_LOG=hot_lib_reloader=trace to see all related logs
    env_logger::init();

    #[cfg(feature = "path-info")]
    path_info::print_paths();

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
        observe::run(
            server_is_running_reader,
            tx_shutdown_server,
            block_reloads_mutex).await
    });

    //main loop
    loop {
        //only run when we can access the mutex
        let lock = block_reloads_mutex_main.lock().await;

        println!("---main loop---");

        main_task::run(server_is_running_writer.clone(), rx_shutdown_server.clone()).await;

        println!("---main loop finished---");

        //only allow more reloads when we are finished
        drop(lock);
    }
}
