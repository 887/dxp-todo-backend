use poem::{get, Route};

mod api;
mod hot;
mod index;
mod swagger_ui;

pub fn get_route() -> Route {
    let route = Route::new()
        .at("/", get(index::index))
        .nest("/hot", hot::get_route()); //routers need to be nested

    //go to http://127.0.0.1:8000/swagger
    #[cfg(feature = "swagger-ui")]
    let route = route.nest("/swagger", swagger_ui::get_route(Some("/api/swagger.json")));

    let api_service = api::get_api_service("http://127.0.0.1:8000");
    route.nest("/api", api::get_route(api_service))
}
