use poem::{get, Route};

mod api;
mod index;

pub fn get_route() -> Route {
    Route::new().at("/", get(index::index))
}
