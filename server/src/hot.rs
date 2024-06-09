//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

use crate::server;
use crate::Result;

use server::run_server_main;

use tracing::trace;

#[no_mangle]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

#[no_mangle]
pub extern "Rust" fn run_server(
    rx_shutdown_server: std::sync::Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<()>>>,
) -> Result<()> {
    #[cfg(feature = "log")]
    let log_subscription = dxp_logging::get_subscription()?;
    let res = Ok(run_server_main(Some(wait_for_shutdown(
        rx_shutdown_server,
    )))?);
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}

async fn wait_for_shutdown(
    rx_shutdown_server: std::sync::Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<()>>>,
) {
    match (rx_shutdown_server).write().await.recv().await {
        Some(_) => {
            trace!("received shutdown_server signal, time to shut down");
        }
        None => {
            trace!("shutdown_server listening channel closed");
        }
    }
}
