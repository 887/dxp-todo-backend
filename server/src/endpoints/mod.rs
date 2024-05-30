use poem::{get, Route};

mod api;
mod index;

pub fn get_route() -> Route {
    let route = Route::new().at("/", get(index::index));
    api::get_route(route)
    // route.nest("/", get(index::index))
}
