//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

use crate::migration;
use crate::Result;

#[no_mangle]
pub extern "Rust" fn run_migration() -> Result<()> {
    #[cfg(feature = "log")]
    let log_subscription = dxp_logging::get_subscription()?;
    let res = migration::run_migration_main();
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}
