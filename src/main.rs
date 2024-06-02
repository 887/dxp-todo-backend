#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send>>;

mod hot_libs;
#[cfg(feature = "hot-reload")]
mod observe;
#[cfg(feature = "path-info")]
mod path_info;

mod main_task;

#[cfg(not(feature = "hot-reload"))]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not load .env"))?;

    #[cfg(feature = "log")]
    let log_subscription = get_log_subscription()?;
    let res = main_task::run().await;
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}

#[cfg(feature = "hot-reload")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    use std::sync::Arc;
    use tokio::sync::mpsc;
    use tokio::sync::{Mutex, RwLock};
    use tracing::trace;

    dotenvy::dotenv()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "could not load .env"))?;

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

    #[cfg(feature = "log")]
    let log_subscription_observe = get_log_subscription()?;
    tokio::task::spawn(async move {
        let res = observe::run(
            server_is_running_reader,
            tx_shutdown_server,
            block_reloads_mutex_task,
        )
        .await;
        #[cfg(feature = "log")]
        drop(log_subscription_observe);
        res
    });

    //main loop
    loop {
        #[cfg(feature = "log")]
        let log_subscription = get_log_subscription()?;

        //only run when we can access the mutex
        let lock = block_reloads_mutex.lock().await;

        trace!("---main loop---");

        main_task::run(server_is_running_writer.clone(), rx_shutdown_server.clone()).await;

        trace!("---main loop finished---");

        //only allow more reloads once finished
        drop(lock);

        #[cfg(feature = "log")]
        drop(log_subscription);
    }
}

#[cfg(feature = "log")]
fn get_log_subscription() -> std::io::Result<logging::LogGuard> {
    logging::get_subscription().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not get log subscription: {:?}", err),
        )
    })
}
