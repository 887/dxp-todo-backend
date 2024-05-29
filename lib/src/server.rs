use std::env;
use std::future::Future;

use anyhow::Context;
use poem::middleware::Compression;
use poem::{listener::TcpListener, Server};
use poem::{EndpointExt, IntoEndpoint};

use crate::endpoints;
use crate::session;

pub fn get_tcp_listener() -> Result<TcpListener<String>, anyhow::Error> {
    let host = env::var("HOST").context("HOST is not set")?;
    let port = env::var("PORT").context("PORT is not set")?;

    let server_url = format!("http://{host}:{port}");

    println!("Starting server at {server_url}");

    Ok(TcpListener::bind(format!("{host}:{port}")))
}

pub fn get_endpoints() -> Result<impl IntoEndpoint + 'static, anyhow::Error> {
    use poem::EndpointExt;

    let main_route = endpoints::get_route();
    let main_route = main_route.with(Compression::new());

    Ok(main_route)
}

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()>>(
    shutdown: Option<F>,
) -> Result<(), anyhow::Error> {
    let tcp_listener = get_tcp_listener()?;
    let endpoints = get_endpoints()?;

    let server = Server::new(tcp_listener);

    let db = dbopen::get_database_connection()
        .await
        .map_err(|e| anyhow::anyhow!("could not get db connection: {}", e))?;

    let session_storage = session::get_db_storage(db.clone()).await?;
    let middleware = session::get_sever_session(session_storage)?;

    let endpoints = endpoints.with(middleware);

    println!("running sever");

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
            println!("server shut down success");
            Ok(())
        }
        Err(err) => {
            println!("server shut down with error: {:?}", err);
            Err(anyhow::anyhow!("server error: {}", err))
        }
    };

    //ensure we always close the database here
    db.close().await?;

    result
}
