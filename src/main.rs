#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

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
    let tx_shutdown_server = Arc::new(RwLock::new(None));
    let tx_shutdown_server_task = tx_shutdown_server.clone();

    //ensures that the server and reloads are blocking
    let block_reloads_mutex = Arc::new(Mutex::new(()));
    let block_reloads_mutex_task = block_reloads_mutex.clone();

    tokio::task::spawn(async move {
        #[cfg(feature = "log")]
        let Ok(log_subscription_observe) = dxp_logging::get_subscription() else {
            return;
        };
        let res = observe::run(tx_shutdown_server_task, block_reloads_mutex_task).await;
        #[cfg(feature = "log")]
        drop(log_subscription_observe);
        res
    });

    //main loop
    loop {
        //only run when we can access the mutex
        let lock = block_reloads_mutex.lock().await;

        let (tx, rx_shutdown_server) = mpsc::channel(1);
        {
            let mut lock = tx_shutdown_server.write().await;
            *lock = Some(tx);
        }

        #[cfg(feature = "log")]
        let log_subscription = get_log_subscription()?;

        trace!("---main loop---");

        main_task::run(rx_shutdown_server).await;

        trace!("---main loop finished---");

        #[cfg(feature = "log")]
        drop(log_subscription);

        //only allow more reloads once finished
        drop(lock);
    }
}

#[cfg(feature = "log")]
fn get_log_subscription() -> std::io::Result<dxp_logging::LogGuard> {
    dxp_logging::get_subscription().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not get log subscription: {:?}", err),
        )
    })
}
