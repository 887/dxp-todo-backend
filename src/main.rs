#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

mod server;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[cfg(feature = "hot-reload")]
fn main() -> std::io::Result<()> {
    server::hot::main()
}

#[cfg(not(feature = "hot-reload"))]
fn main() -> std::io::Result<()> {
    server::cold::main()
}
