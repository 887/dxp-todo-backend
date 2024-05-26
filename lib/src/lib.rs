#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

mod endpoints;

mod server;

use anyhow::Context;
use server::run_server_main;

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf, anyhow::Error> {
    dotenvy::dotenv_override().context("could not load .env")
}

#[cfg(not(debug_assertions))]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf, anyhow::Error> {
    dotenvy::dotenv().context("could not load .env")
}

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn run_server(
    rx_shutdown_server: std::sync::Arc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<()>>>,
) -> Result<(), anyhow::Error> {
    run_server_main(wait_for_shutdown(rx_shutdown_server), Some(()))
}

#[cfg(not(debug_assertions))]
pub extern "Rust" fn run_server() -> Result<(), anyhow::Error> {
    let empty = async {};
    run_server_main(empty, None::<_>)
}

#[cfg(debug_assertions)]
async fn wait_for_shutdown(rx_shutdown_server: sea_orm::prelude::RcOrArc<tokio::sync::RwLock<tokio::sync::mpsc::Receiver<()>>>) {
    match (rx_shutdown_server).write().await.recv().await {
        Some(_) => {
            println!("received shutdown_server signal, time to shut down");
        }
        None => {
            println!("shutdown_server listening channel closed");
        }
    }
}

