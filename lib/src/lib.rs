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

#[cfg(feature = "migration")]
mod migration;
mod server;

use anyhow::Context;
use server::run_server_main;

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf, anyhow::Error> {
    #[cfg(debug_assertions)]
    return dotenvy::dotenv_override().context("could not load .env");
    #[cfg(not(debug_assertions))]
    dotenvy::dotenv().context("could not load .env")
}

#[cfg(feature = "migration")]
#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn run_migration() -> Result<(), anyhow::Error> {
    migration::run_migration_main()
}

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn run_server(
    mut rx_shutdown_server: tokio::sync::mpsc::Receiver<()>,
) -> Result<(), anyhow::Error> {
    let shutdown_received = async move {
        match (rx_shutdown_server).recv().await {
            Some(_) => {
                println!("received shutdown_server signal, time to shut down");
            }
            None => {
                println!("shutdown_server listening channel closed");
            }
        }
    };
    run_server_main(shutdown_received)
}
