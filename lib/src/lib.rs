mod endpoints;

use std::env;

use anyhow::anyhow;
use poem::middleware::Compression;
use poem::Route;
use poem::{listener::TcpListener, Server};
use std::convert::Infallible;

#[cfg(any(feature = "migration"))]
mod migration;

#[cfg(debug_assertions)]
#[no_mangle]
pub fn get_assembled_server() -> Result<Server<TcpListener<String>, Infallible>, anyhow::Error> {
    load_env();

    //todo, open db etc
    #[cfg(any(feature = "migration"))]
    migration::run_migration();

    let host = match env::var("HOST") {
        Ok(res) => res,
        Err(_err) => return Err(anyhow!("HOST is not set in .env file")),
    };
    let port = match env::var("PORT") {
        Ok(res) => res,
        Err(_err) => return Err(anyhow!("PORT is not set in .env file")),
    };

    let server_url = format!("http://{host}:{port}");

    println!("Starting server at {server_url}");

    let server = Server::new(TcpListener::bind(format!("{host}:{port}")));
    Ok(server)
}

#[cfg(debug_assertions)]
#[no_mangle]
pub fn get_endpoints() -> Result<Route, anyhow::Error> {
    use poem::EndpointExt;

    load_env();

    let main_route = endpoints::get_route();
    let main_route = main_route.with(Compression::new());

    let route = Route::new().at("/", main_route);

    Ok(route)
}

fn load_env() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv_override().ok();
    #[cfg(not(debug_assertions))]
    dotenvy::dotenv().ok();
}
