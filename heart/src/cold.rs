use server::run_server_main;

use crate::server;
use crate::Result;

pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

#[cfg(feature = "log")]
pub extern "Rust" fn run_server() -> Result<()> {
    let log_dispatcher = dxp_logging::get_subscriber()
        .map_err(|e| anyhow::anyhow!("could not get log subscriber: {}", e))?
        .get_dispatcher();
    let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher);

    let empty = None::<Option<()>>.map(|_| async {});
    let res = Ok(run_server_main(empty, &log_dispatcher)?);

    drop(log_guard);
    res
}

#[cfg(not(feature = "log"))]
pub extern "Rust" fn run_server() -> Result<()> {
    let empty = None::<Option<()>>.map(|_| async {});
    run_server_main(empty)
}
