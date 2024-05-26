#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

mod migration;

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn run_migration() -> Result<(), anyhow::Error> {
    migration::run_migration_main()
}
