#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send>>;

#[cfg(feature = "hot-reload")]
use std::sync::Arc;

#[cfg(feature = "hot-reload")]
use tokio::sync::mpsc;
#[cfg(feature = "hot-reload")]
use tokio::sync::{Mutex, RwLock};

mod hot_libs;
#[cfg(feature = "hot-reload")]
mod observe;
mod path_info;

mod main_task;

#[cfg(not(feature = "hot-reload"))]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    main_task::run().await
}

#[cfg(feature = "hot-reload")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Use RUST_LOG=hot_lib_reloader=trace to see all related logs
    // env_logger::init();

    #[cfg(feature = "path-info")]
    path_info::print_paths();

    //this channel is to shut down the server
    let (tx_shutdown_server, rx_shutdown_server) = mpsc::channel(1);
    let rx_shutdown_server = Arc::new(RwLock::new(rx_shutdown_server));

    //ensures that the server and reloads are blocking
    let block_reloads_mutex = Arc::new(Mutex::new(0));

    //check if the server is running, avoid sending messages to an inactive server
    let server_is_running = Arc::new(RwLock::new(false));
    let server_is_running_writer = server_is_running.clone();

    let block_reloads_mutex_task = block_reloads_mutex.clone();
    let server_is_running_reader = server_is_running.clone();

    tokio::task::spawn(async move {
        observe::run(
            server_is_running_reader,
            tx_shutdown_server,
            block_reloads_mutex_task,
        )
        .await
    });

    //main loop
    loop {
        //only run when we can access the mutex
        let lock = block_reloads_mutex.lock().await;

        println!("---main loop---");

        main_task::run(server_is_running_writer.clone(), rx_shutdown_server.clone()).await;

        println!("---main loop finished---");

        //only allow more reloads once finished
        drop(lock);
    }
}
