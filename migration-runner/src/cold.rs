use crate::migration;
use crate::Result;

pub extern "Rust" fn run_migration() -> Result<()> {
    #[cfg(feature = "log")]
    let log_subscription = dxp_logging::get_subscription()?;
    let res = migration::run_migration_main();
    #[cfg(feature = "log")]
    drop(log_subscription);
    res
}
