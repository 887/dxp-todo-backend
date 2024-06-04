#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod migration;

#[cfg(feature = "hot-reload")]
#[no_mangle]
pub extern "Rust" fn run_migration() -> Result<()> {
    #[cfg(feature = "log")]
    let log_subscription = dxp_logging::get_subscription()?;
    let res = migration::run_migration_main();
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}

#[cfg(not(feature = "hot-reload"))]
pub extern "Rust" fn run_migration() -> Result<()> {
    #[cfg(feature = "log")]
    let log_subscription = dxp_logging::get_subscription()?;
    let res = migration::run_migration_main();
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}
