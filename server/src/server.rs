use std::env;
use std::future::Future;
use std::net::Ipv4Addr;
use std::sync::Arc;
use std::sync::RwLock;

use anyhow::Context;
use anyhow::Result;
use tokio::runtime::Builder;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::error;
use tracing::info;
use tracing::trace;

use crate::endpoint;
use crate::tracing_layer::TracingLayer;

pub async fn get_tcp_listener() -> Result<TcpListener> {
    let host = env::var("HOST").context("HOST is not set")?;
    let port = env::var("PORT").context("PORT is not set")?;

    let server_url = format!("http://{host}:{port}");

    info!("Starting server at {server_url}");

    let port: u16 = port.parse().context("PORT is not a valid number")?;
    let address = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);

    Ok(TcpListener::bind(&address).await?)
}

pub fn run_server_main<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
    log_dispatcher: &dxp_logging::LogDispatcher,
) -> Result<()> {
    let hash_map = Arc::new(RwLock::new(std::collections::HashMap::new()));
    let hash_map_clone_open = hash_map.clone();
    let hash_map_clone_close = hash_map.clone();
    let log_dispatcher_clone = log_dispatcher.clone();
    let runtime = Builder::new_multi_thread()
        .on_thread_start(move || {
            let hash_map = hash_map_clone_open.clone();
            // Initialize thread-local resource for each thread
            let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher_clone);
            let thread_id = std::thread::current().id();
            hash_map.write().unwrap().insert(thread_id, log_guard);
        })
        .on_thread_stop(move || {
            let hash_map = hash_map_clone_close.clone();
            let thread_id = std::thread::current().id();
            hash_map.write().unwrap().remove(&thread_id);
        })
        .enable_all()
        .build()?;

    runtime.block_on(async { run_server_main_inner(shutdown, log_dispatcher).await })
}

//https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
// #[tokio::main]
pub async fn run_server_main_inner<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
    log_dispatcher: &dxp_logging::LogDispatcher,
) -> Result<()> {
    let listener = get_tcp_listener().await?;

    let db = dxp_db_open::get_database_connection()
        .await
        .map_err(|e| anyhow::anyhow!("could not get db connection: {}", e))?;

    let app = endpoint::get_route(db.clone()).await?;

    #[cfg(feature = "log")]
    let app = app.layer(TracingLayer {
        log_dispatcher: log_dispatcher.clone(),
    });

    #[cfg(feature = "log")]
    let app = app.layer(TraceLayer::new_for_http());

    info!("running sever");

    let server = axum::serve(listener, app);
    let run_result = match shutdown {
        Some(shutdown) => {
            let graceful = server.with_graceful_shutdown(shutdown);
            graceful.await
        }
        None => server.await,
    };

    let result = match run_result {
        Ok(_) => {
            trace!("server shut down success");
            Ok(())
        }
        Err(err) => {
            error!("server shut down with error: {:?}", err);
            Err(anyhow::anyhow!("server error: {}", err))
        }
    };

    //ensure we always close the database here
    db.close().await?;

    result
}
