mod endpoints;

use std::env;

use anyhow::Context;
use poem::middleware::Compression;
use poem::Route;
use poem::{listener::TcpListener, Server};
use std::convert::Infallible;
use std::sync::Arc;

#[cfg(any(feature = "migration"))]
#[cfg(debug_assertions)]
#[no_mangle]
pub fn run_migration(rt: Arc<Box<tokio::runtime::Handle>>, db_url: &str) -> Box<tokio::task::JoinHandle<()>> {
    println!("Running migration {db_url}");

    migration_runner::run_migration(rt, db_url)
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn get_assembled_server(
) -> Result<Server<TcpListener<String>, Infallible>, anyhow::Error> {
    let host = env::var("HOST").context("HOST is not set in .env file")?;
    let port = env::var("PORT").context("PORT is not set in .env file")?;

    let server_url = format!("http://{host}:{port}");

    println!("Starting server at {server_url}");

    let server = Server::new(TcpListener::bind(format!("{host}:{port}")));
    Ok(server)
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn async_should_do_async_thing(
    rt: Arc<Box<tokio::runtime::Handle>>,
) -> () {
    rt.block_on(async {
        println!("doing async thing");
        tokio::time::sleep(std::time::Duration::from_secs(0)).await
    })
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
pub fn load_env() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv_override().ok();
    #[cfg(not(debug_assertions))]
    dotenvy::dotenv().ok();
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn get_database_url() -> Result<String, anyhow::Error> {
    Ok(env::var("DATABASE_URL").context("DATABASE_URL is not set in .env file")?)
}
