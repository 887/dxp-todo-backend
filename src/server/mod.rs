#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic
)]

#[cfg(feature = "hot-reload")]
mod observe;
#[cfg(feature = "path-info")]
mod path_info;

#[cfg(feature = "hot-reload")]
mod hot;
#[cfg(feature = "hot-reload")]
pub use hot::*;

#[cfg(not(feature = "hot-reload"))]
mod cold;
#[cfg(not(feature = "hot-reload"))]
pub use cold::*;

#[cfg(feature = "log")]
fn get_log_subscription() -> std::io::Result<dxp_logging::LogGuard> {
    dxp_logging::get_subscription().map_err(|err| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("could not get log subscription: {:?}", err),
        )
    })
}

#[cfg(feature = "migration")]
pub async fn run_migrations() -> crate::Result<()> {
    Ok(tokio::task::spawn_blocking(|| {
        hot_migration_runner::run_migration()
            .map_err(|e| format!("migration aborted with error, {:?}", e))
    })
    .await??)
}
