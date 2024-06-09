#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

mod endpoints;
mod server;

#[cfg(not(feature = "hot-reload"))]
mod cold;
#[cfg(not(feature = "hot-reload"))]
pub use cold::*;
#[cfg(feature = "hot-reload")]
mod hot;
#[cfg(feature = "hot-reload")]
pub use hot::*;
