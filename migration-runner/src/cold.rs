use crate::migration;
use crate::Result;

pub extern "Rust" fn run_migration() -> Result<()> {
    #[cfg(feature = "log")]
    let log_subscription = dxp_logging::subscribe_thread_with_default()?;
    let res = migration::run_migration_main();
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}
