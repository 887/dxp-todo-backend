mod endpoints;

use std::env;

use anyhow::Context;
use poem::middleware::Compression;
use poem::Route;
use poem::{listener::TcpListener, Server};
use std::convert::Infallible;
use std::future::IntoFuture;
use std::future::Future;
use std::pin::Pin;
use std::pin::pin;

#[cfg(any(feature = "migration"))]
#[cfg(debug_assertions)]
#[no_mangle]
pub fn run_migration() -> Result<(), anyhow::Error> {
    run_migration_inner()
}

//https://stackoverflow.com/questions/62536566/how-can-i-create-a-tokio-runtime-inside-another-tokio-runtime-without-getting-th
#[tokio::main]
pub async fn run_migration_inner() -> Result::<(), anyhow::Error> {
    println!("Running migration");

    let db = dbopen::get_database_connection().await.context("could not get db connection")?;

    migration_runner::run_migrator(&db).await.context("migration failed")?;

    db.close().await?;
    Result::<(), anyhow::Error>::Ok(())
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn get_assembled_server() -> Result<Server<TcpListener<String>, Infallible>, anyhow::Error> {
    let host = env::var("HOST").context("HOST is not set in .env file")?;
    let port = env::var("PORT").context("PORT is not set in .env file")?;

    let server_url = format!("http://{host}:{port}");

    println!("Starting server at {server_url}");

    let server = Server::new(TcpListener::bind(format!("{host}:{port}")));
    Ok(server)
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn get_endpoints() -> Result<Route, anyhow::Error> {
    use poem::EndpointExt;

    let main_route = endpoints::get_route();
    let main_route = main_route.with(Compression::new());

    let route = Route::new().at("/", main_route);

    Ok(route)
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn load_env() -> Result<std::path::PathBuf, anyhow::Error> {
    #[cfg(debug_assertions)]
    return dotenvy::dotenv_override().context("could not load .env");
    #[cfg(not(debug_assertions))]
    dotenvy::dotenv().context("could not load .env")
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn run_server()  -> Result<(), anyhow::Error> {
    Ok(())
}
