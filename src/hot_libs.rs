#[hot_lib_reloader::hot_module(dylib = "lib")]
pub(crate) mod hot_lib {
    // pub use lib::*;

    hot_functions_from_file!("lib/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[hot_lib_reloader::hot_module(dylib = "migration_runner")]
pub(crate) mod hot_migration_runner {
    // pub use lib::*;

    hot_functions_from_file!("migration-runner/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}
