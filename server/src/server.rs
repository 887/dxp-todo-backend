use std::env;
use std::future::Future;
use std::net::Ipv4Addr;

use anyhow::Context;
use anyhow::Result;

use axum_server::Server;
use tokio::net::unix::SocketAddr;
use tokio::net::TcpListener;
use tracing::error;
use tracing::info;
use tracing::trace;

use crate::endpoint;

pub fn get_tcp_listener() -> Result<TcpListener> {
    let host = env::var("HOST").context("HOST is not set")?;
    let port = env::var("PORT").context("PORT is not set")?;

    let server_url = format!("http://{host}:{port}");

    info!("Starting server at {server_url}");

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));

    Ok(TcpListener::bind(&address))
}

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()>>(shutdown: Option<F>) -> Result<()> {
    let listener = get_tcp_listener()?;

    // let server = Server::new(tcp_listener);

    let db = dxp_db_open::get_database_connection()
        .await
        .map_err(|e| anyhow::anyhow!("could not get db connection: {}", e))?;

    let app = endpoint::get_route(db.clone()).await?;

    info!("running sever");

    let run_result = match shutdown {
        Some(shutdown) => {
            Server::from_tcp(listener)
                .serve(app)
                .with_graceful_shutdown(shutdown)
                .await
        }
        None => Server::from_tcp(listener).serve(app).await,
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
