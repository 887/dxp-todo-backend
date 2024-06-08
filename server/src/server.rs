use std::env;
use std::future::Future;

use anyhow::Context;
use anyhow::Result;

use poem::{listener::TcpListener, Server};
use poem::{EndpointExt, IntoEndpoint};
use sea_orm::DatabaseConnection;
use tracing::error;
use tracing::info;
use tracing::trace;

use crate::endpoints;

pub fn get_tcp_listener() -> Result<TcpListener<String>> {
    let host = env::var("HOST").context("HOST is not set")?;
    let port = env::var("PORT").context("PORT is not set")?;

    let server_url = format!("http://{host}:{port}");

    info!("Starting server at {server_url}");

    Ok(TcpListener::bind(format!("{host}:{port}")))
}

pub async fn get_endpoints(db: DatabaseConnection) -> Result<impl IntoEndpoint + 'static> {
    let main_route = endpoints::get_route(db.clone()).await?;

    Ok(main_route)
}

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()>>(shutdown: Option<F>) -> Result<()> {
    let tcp_listener = get_tcp_listener()?;

    let server = Server::new(tcp_listener);

    let db = dxp_db_open::get_database_connection()
        .await
        .map_err(|e| anyhow::anyhow!("could not get db connection: {}", e))?;

    let endpoints = get_endpoints(db.clone()).await?;

    info!("running sever");

    let run_result = match shutdown {
        Some(shutdown) => {
            server
                .run_with_graceful_shutdown(endpoints, shutdown, None)
                .await
        }
        None => server.run(endpoints).await,
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
