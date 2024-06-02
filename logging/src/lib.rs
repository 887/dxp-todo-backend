use tracing::Level;
use tracing_subscriber::filter;

//not - can't use     tracing_subscriber::fmt::init();
//anything using the global scope is incompatible with hot-reload and defeats the purpose

pub fn get_subscriber() -> tracing_subscriber::FmtSubscriber<
    tracing_subscriber::fmt::format::DefaultFields,
    tracing_subscriber::fmt::format::Format,
    tracing_subscriber::EnvFilter,
> {
    tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        .with_env_filter(filter::EnvFilter::from_default_env())
        // build but do not install the subscriber.
        .finish()
}

// fn enable_log() -> std::io::Result<()> {
//     #[cfg(all(feature = "log", not(feature = "log-file")))]
//     tracing_subscriber::fmt::init();

//     #[cfg(all(feature = "log", feature = "log-file"))]
//     {
//         let log_dir = unwrap_ok_or!(env::var("LOG_PATH"), _err, {
//             return Err(Error::new(
//                 ErrorKind::Other,
//                 "LOG_PATH is not set in .env file",
//             ));
//         });

//         let log_prefix = unwrap_ok_or!(env::var("LOG_PREFIX"), _err, {
//             return Err(Error::new(
//                 ErrorKind::Other,
//                 "LOG_PREFIX is not set in .env file",
//             ));
//         });

//         //https://docs.rs/tracing-appender/latest/tracing_appender/
//         let file_appender = tracing_appender::rolling::daily(log_dir, log_prefix);
//         let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
//         tracing_subscriber::fmt().with_writer(non_blocking).init();
//     }

//     Ok(())
// }
