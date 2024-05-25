mod endpoints;

use std::env;

use anyhow::Context;
use poem::middleware::Compression;
use poem::Route;
use poem::{listener::TcpListener, Server};
use std::convert::Infallible;

use sea_orm::DatabaseConnection;

#[cfg(any(feature = "migration"))]
#[cfg(debug_assertions)]
#[no_mangle]
pub fn run_migration(
    rt: tokio::runtime::Handle,
    db: DatabaseConnection,
) -> Result<(), anyhow::Error> {
    println!("Running migration");

    migration_runner::run_migration(rt, db)
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
