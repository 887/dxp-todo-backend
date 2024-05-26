#[cfg(debug_assertions)]
#[hot_lib_reloader::hot_module(dylib = "lib")]
pub(crate) mod hot_lib {
    // pub use lib::*;

    hot_functions_from_file!("lib/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[cfg(debug_assertions)]
#[hot_lib_reloader::hot_module(dylib = "migration_runner")]
pub(crate) mod hot_migration_runner {
    // pub use migration_runner::*;

    hot_functions_from_file!("migration-runner/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[cfg(not(debug_assertions))]
pub(crate) mod hot_lib {
    pub(crate) fn run_server() -> Result<(), anyhow::Error> {
        lib::run_server()
    }

    pub(crate) fn load_env() -> Result<std::path::PathBuf, anyhow::Error> {
        lib::load_env()
    }
}

#[cfg(not(debug_assertions))]
pub(crate) mod hot_migration_runner {
    pub(crate) fn run_migration() -> Result<(), anyhow::Error> {
        migration_runner::run_migration()
    }
}
