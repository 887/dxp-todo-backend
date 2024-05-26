use std::env;
use std::future::Future;

use anyhow::Context;
use poem::middleware::Compression;
use poem::Route;
use poem::{listener::TcpListener, Server};

use crate::endpoints;

pub fn get_tcp_listener() -> Result<TcpListener<String>, anyhow::Error> {
    let host = env::var("HOST").context("HOST is not set in .env file")?;
    let port = env::var("PORT").context("PORT is not set in .env file")?;

    let server_url = format!("http://{host}:{port}");

    println!("Starting server at {server_url}");

    Ok(TcpListener::bind(format!("{host}:{port}")))
}

pub fn get_endpoints() -> Result<Route, anyhow::Error> {
    use poem::EndpointExt;

    let main_route = endpoints::get_route();
    let main_route = main_route.with(Compression::new());

    let route = Route::new().at("/", main_route);

    Ok(route)
}

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_server_main<F: Future<Output = ()>>(shutdown: F) -> Result::<(), anyhow::Error> {

    let tcp_listener = get_tcp_listener()?;
    let endpoints = get_endpoints()?;

    let server = Server::new(tcp_listener);

    let db = dbopen::get_database_connection().await.context("could not get db connection")?;

    println!("running sever");

    let run_result = server.run_with_graceful_shutdown(endpoints, shutdown, None).await;
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
