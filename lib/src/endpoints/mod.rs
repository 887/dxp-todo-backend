use poem::{get, Route};

mod api;
mod index;

pub fn get_route() -> Route {
    let route = Route::new();
    let route = api::get_route(route);
    let route = route.nest("/pi", get(index::index));

    route
    // route.nest("/", get(index::index))
}
