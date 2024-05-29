#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod endpoints;
mod server;
mod session;

use server::run_server_main;

#[cfg(feature = "hot-reload")]
#[no_mangle]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

#[cfg(not(feature = "hot-reload"))]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv_override().map_err(|_| "could not load .env")?)
}

#[cfg(feature = "hot-reload")]
#[no_mangle]
pub extern "Rust" fn run_server(
    rx_shutdown_server: std::sync::Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<()>>>,
) -> Result<()> {
    Ok(run_server_main(Some(wait_for_shutdown(
        rx_shutdown_server,
    )))?)
}

#[cfg(not(feature = "hot-reload"))]
pub extern "Rust" fn run_server() -> Result<()> {
    let empty = None::<Option<()>>.map(|_| async {});
    Ok(run_server_main(empty)?)
}

#[cfg(feature = "hot-reload")]
async fn wait_for_shutdown(
    rx_shutdown_server: std::sync::Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<()>>>,
) {
    match (rx_shutdown_server).write().await.recv().await {
        Some(_) => {
            println!("received shutdown_server signal, time to shut down");
        }
        None => {
            println!("shutdown_server listening channel closed");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use anyhow::Result;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test1() -> Result<()> {
        sleep(Duration::from_secs(2)).await;
        Ok(())
    }
}
