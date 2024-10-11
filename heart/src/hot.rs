//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

use std::any::Any;

use crate::server;
use crate::Result;

use server::run_server_main;

use tracing::{error, trace};

#[no_mangle]
pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

#[no_mangle]
pub extern "Rust" fn run_server(rx_shutdown_server: tokio::sync::mpsc::Receiver<()>) -> Result<()> {
    let unwind_result = std::panic::catch_unwind(move || {
        #[cfg(feature = "log")]
        {
            let log_dispatcher = dxp_logging::get_subscriber()
                .map_err(|e| anyhow::anyhow!("could not get log subscriber: {}", e))?
                .get_dispatcher();
            let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher);

            let res = run_server_main(Some(wait_for_shutdown(rx_shutdown_server)), &log_dispatcher);

            drop(log_guard);
            res
        }
        #[cfg(not(feature = "log"))]
        run_server_main(Some(wait_for_shutdown(rx_shutdown_server)));
    });

    #[cfg(feature = "log")]
    let log_guard = dxp_logging::subscribe_thread_with_default();

    let res = match unwind_result {
        Ok(res) => res,
        Err(err) => {
            let err = get_unwound_error(err);
            #[cfg(feature = "log")]
            error!(
                "run_server panicked: \n\
                {:?}",
                err
            );
            Err(err)
        }
    };

    #[cfg(feature = "log")]
    drop(log_guard);

    Ok(res?)
}

async fn wait_for_shutdown(mut rx_shutdown_server: tokio::sync::mpsc::Receiver<()>) {
    match (rx_shutdown_server).recv().await {
        Some(_) => {
            trace!("received shutdown_server signal, time to shut down");
        }
        None => {
            trace!("shutdown_server listening channel closed");
        }
    }
}

pub fn get_unwound_error(err: Box<dyn Any + Send>) -> anyhow::Error {
    if err.is::<String>() {
        if let Ok(err) = err.downcast::<String>() {
            anyhow::anyhow!("Unhandled Error: {:?}", err)
        } else {
            anyhow::anyhow!("Unhandled Error!")
        }
    } else if err.is::<&str>() {
        if let Ok(err) = err.downcast::<&str>() {
            anyhow::anyhow!("Unhandled Error: {:?}", err)
        } else {
            anyhow::anyhow!("Unhandled Error!")
        }
    } else {
        anyhow::anyhow!("Unhandled Error: {:?}", err)
    }
}
