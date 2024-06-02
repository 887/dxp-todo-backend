use tracing_subscriber::filter;

//not - can't use     tracing_subscriber::fmt::init();
//anything that pins itself to memory is incompatible with hot-reload and defeats the purpose

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub type LogGuard = tracing::subscriber::DefaultGuard;

#[cfg(all(feature = "log", feature = "log-file"))]
pub type LogGuard = (
    tracing::subscriber::DefaultGuard,
    tracing_appender::non_blocking::WorkerGuard,
);

#[cfg(all(feature = "log", not(feature = "log-file")))]
pub fn get_subscription() -> Result<tracing::subscriber::DefaultGuard> {
    let log_subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        // build but do not install the subscriber.
        .finish();

    Ok(tracing::subscriber::set_default(log_subscriber))
}

#[cfg(all(feature = "log", feature = "log-file"))]
pub fn get_subscription() -> Result<LogGuard> {
    let log_dir = std::env::var("LOG_PATH").map_err(|e| format!("LOG_PATH is not set {:?}", e))?;
    let log_prefix =
        std::env::var("LOG_PREFIX").map_err(|e| format!("LOG_PREFIX is not set {:?}", e))?;

    //https://docs.rs/tracing-appender/latest/tracing_appender/
    let file_appender = tracing_appender::rolling::daily(log_dir, log_prefix);
    let (non_blocking, file_flush_guard) = tracing_appender::non_blocking(file_appender);

    //https://stackoverflow.com/questions/73225598/why-is-nothing-printed-to-the-terminal-when-using-tracing-appender
    //tracing-appender only logs to the file when enabled, to also log to standard output you need to add that as well
    //let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());

    let log_subscriber = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        .with_writer(non_blocking)
        // build but do not install the subscriber.
        .finish();

    let subscriber_guard = tracing::subscriber::set_default(log_subscriber);
    Ok((subscriber_guard, file_flush_guard))
}
