#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]
//#[no_mangle] is unsafe, but needed for hot reload.
//https://github.com/rust-lang/rust/issues/111967
#![allow(unsafe_code)]

mod migration;

#[cfg(debug_assertions)]
#[no_mangle]
pub extern "Rust" fn run_migration() -> Result<(), anyhow::Error> {
    migration::run_migration_main()
}
