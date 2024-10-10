use server::run_server_main;

use crate::server;
use crate::Result;

pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

pub extern "Rust" fn run_server() -> Result<()> {
    #[cfg(feature = "log")]
    let log_dispatcher = dxp_logging::get_subscriber()
        .map_err(|e| anyhow::anyhow!("could not get log subscriber: {}", e))?
        .get_dispatcher();
    #[cfg(feature = "log")]
    let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher);

    let empty = None::<Option<()>>.map(|_| async {});
    let res = Ok(run_server_main(empty)?);

    #[cfg(feature = "log")]
    drop(log_guard);
    res
}
