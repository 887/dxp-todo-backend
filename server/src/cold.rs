use server::run_server_main;

use crate::server;
use crate::Result;

pub extern "Rust" fn load_env() -> Result<std::path::PathBuf> {
    Ok(dotenvy::dotenv().map_err(|_| "could not load .env")?)
}

pub extern "Rust" fn run_server() -> Result<()> {
    let empty = None::<Option<()>>.map(|_| async {});
    let res = Ok(run_server_main(empty)?);
    res
}
