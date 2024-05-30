pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

//info: in order to cause a reload you nee to actually change a function signature/make the compiler do work
//if the file is identical to the compiler, hot-reload will not try to do a reload

#[cfg(feature = "hot-reload")]
#[hot_lib_reloader::hot_module(dylib = "server", file_watch_debounce = 10)]
pub(crate) mod hot_server {
    // pub use lib::*;
    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    hot_functions_from_file!("server/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[cfg(feature = "hot-reload")]
#[hot_lib_reloader::hot_module(dylib = "migration_runner", file_watch_debounce = 10)]
pub(crate) mod hot_migration_runner {
    // pub use migration_runner::*;
    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    hot_functions_from_file!("migration-runner/src/lib.rs");

    // expose a type to subscribe to lib load events
    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

#[cfg(not(feature = "hot-reload"))]
pub(crate) mod hot_server {
    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    pub(crate) fn run_server() -> Result<()> {
        server::run_server()
    }

    pub(crate) fn load_env() -> Result<std::path::PathBuf> {
        server::load_env()
    }
}

#[cfg(feature = "migration")]
#[cfg(not(feature = "hot-reload"))]
pub(crate) mod hot_migration_runner {
    pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    pub(crate) fn run_migration() -> Result<()> {
        migration_runner::run_migration()
    }
}
