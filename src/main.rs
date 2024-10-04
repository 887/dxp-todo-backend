#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

mod server;

#[allow(dead_code)]
pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> std::io::Result<()> {
    server::main()
}
