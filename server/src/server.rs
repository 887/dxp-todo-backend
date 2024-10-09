use std::env;
use std::future::Future;
use std::net::Ipv4Addr;

use anyhow::Context;
use anyhow::Result;

use std::net::SocketAddr;
use tokio::net::TcpListener;
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

//https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()> + Send + 'static>(
    shutdown: Option<F>,
) -> Result<()> {
    #[cfg(feature = "log")]
    let log_dispatcher = dxp_logging::get_subscriber()
        .map_err(|e| anyhow::anyhow!("could not get log subscriber: {}", e))?
        .get_dispatcher();
    #[cfg(feature = "log")]
    let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher);

    let listener = get_tcp_listener().await?;

    let db = dxp_db_open::get_database_connection()
        .await
        .map_err(|e| anyhow::anyhow!("could not get db connection: {}", e))?;

    let app = endpoint::get_route(db.clone()).await?;

    #[cfg(feature = "log")]
    let app = app.layer(TracingLayer {
        log_dispatcher: log_dispatcher.clone(),
    });

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

    #[cfg(feature = "log")]
    drop(log_guard);

    result
}
