use poem::{get, Route};

mod api;
mod hot;
mod index;

pub fn get_route() -> Route {
    Route::new()
        .at("/", get(index::index))
        .nest("/hot", hot::get_route()) //routers need to be nested
        .nest("/api", api::get_route("http://127.0.0.1:8000")) //routers need to be nested
}
