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

    Router::new()
        .route("/", get(swagger_ui_embed::get_html(options)))
        .route(
            "/oauth-receiver.html",
            get(swagger_ui_embed::get_oauth_receiver_html()),
        )
}

fn get_refresh_script() -> &'static str {
    include_str!("refresh.js")
}
