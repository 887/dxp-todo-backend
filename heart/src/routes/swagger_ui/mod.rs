use axum::{response::Html, routing::get, Router};

pub fn get_route(router: Router, url: Option<&str>) -> Router {
    let script = Some(get_refresh_script());

    // could also use this:
    // https://github.com/tyrchen/axum-swagger-ui/tree/master/src

    let options = swagger_ui_embed::Options {
        url,
        script,
        persist_authorization: Some(true),
        // ..Default::default()
    };

    let html = swagger_ui_embed::get_html(options);
    let oauth_receiver_html = swagger_ui_embed::get_oauth2_redirect_html();

    let html_route = get(move || async { Html(html) });
    router
        .route("/swagger", html_route.clone())
        .route("/swagger/", html_route.clone())
        .route("/swagger/index.html", html_route.clone())
        .route(
            "/swagger/oauth-receiver.html",
            get(move || async { Html(oauth_receiver_html) }),
        )
}

fn get_refresh_script() -> &'static str {
    include_str!("refresh.js")
}
