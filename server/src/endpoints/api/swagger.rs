use poem::{Endpoint, Route};
use poem_openapi::OpenApiService;

use super::endpoints::Api;

pub fn get_route(url: Option<&str>) -> impl Endpoint {
    let options = swagger_ui_embed::Options {
        url: url,
        script: Some(get_refresh_script()),
        // ..Default::default()
    };

    Route::new().nest("/", swagger_ui_embed::create_endpoint(options))
}

fn get_refresh_script() -> &'static str {
    include_str!("refresh.js")
}
