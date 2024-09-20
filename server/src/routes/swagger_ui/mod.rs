use axum::{routing::get, Router};

pub fn get_route(url: Option<&str>) -> Router {
    let script = Some(get_refresh_script());

    // could also use this:
    // https://github.com/tyrchen/axum-swagger-ui/tree/master/src

    let options = swagger_ui_embed::Options {
        url,
        script,
        persist_authorization: Some(true),
        // ..Default::default()
    };

    Router::new().route("/", get(swagger_ui_embed::create_endpoint(options)))
}

fn get_refresh_script() -> &'static str {
    include_str!("refresh.js")
}
