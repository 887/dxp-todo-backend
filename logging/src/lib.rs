use tracing_subscriber::filter;

//not - can't use     tracing_subscriber::fmt::init();
//anything that pins itself to memory is incompatible with hot-reload and defeats the purpose

pub type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(all(feature = "log", not(feature = "log-file")))]
fn get_subscriber() -> Result<
    tracing_subscriber::FmtSubscriber<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        tracing_subscriber::EnvFilter,
    >,
> {
    Ok(tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        // build but do not install the subscriber.
        .finish())
}

#[cfg(all(feature = "log", feature = "log-file"))]
fn get_subscriber() -> Result<
    tracing_subscriber::FmtSubscriber<
        tracing_subscriber::fmt::format::DefaultFields,
        tracing_subscriber::fmt::format::Format,
        filter::EnvFilter,
        tracing_appender::non_blocking::NonBlocking,
    >,
> {
    let log_dir = std::env::var("LOG_PATH").map_err(|e| format!("LOG_PATH is not set {:?}", e))?;
    let log_prefix =
        std::env::var("LOG_PREFIX").map_err(|e| format!("LOG_PREFIX is not set {:?}", e))?;

    //https://docs.rs/tracing-appender/latest/tracing_appender/
    let file_appender = tracing_appender::rolling::daily(log_dir, log_prefix);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    Ok(tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        // .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        .with_writer(non_blocking)
        // build but do not install the subscriber.
        .finish())
}

#[cfg(any(feature = "log", feature = "log-file"))]
pub fn get_subscription() -> Result<Option<tracing::subscriber::DefaultGuard>> {
    let log_subscriber = get_subscriber()?;
    Ok(Some(tracing::subscriber::set_default(log_subscriber)))
}
