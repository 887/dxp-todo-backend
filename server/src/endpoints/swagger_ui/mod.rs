use poem::{Endpoint, Route};

pub fn get_route(url: Option<&str>) -> impl Endpoint {
    let script = Some(get_refresh_script());
    let options = swagger_ui_embed::Options {
        url,
        script,
        // ..Default::default()
    };

    Route::new().nest("/", swagger_ui_embed::create_endpoint(options))
}

fn get_refresh_script() -> &'static str {
    include_str!("refresh.js")
}
