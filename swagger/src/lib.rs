use poem::{endpoint::make_sync, web::Html, Endpoint};

const SWAGGER_UI_JS: &str = include_str!("swagger-ui-bundle.js");
const SWAGGER_UI_CSS: &str = include_str!("swagger-ui.css");
const OAUTH_RECEIVER_HTML: &str = include_str!("oauth-receiver.html");

//https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/
const SWAGGER_UI_TEMPLATE: &str = r#"
<html charset="UTF-8">
<head>
<meta http-equiv="Content-Type" content="text/html;charset=utf-8">
<title>Swagger UI</title>
<style charset="UTF-8">{:style}</style>
<script charset="UTF-8">{:script}</script>
</head>
<body>

<div id="ui"></div>
<script>
    let oauth2RedirectUrl;

    let query = window.location.href.indexOf("?");
    if (query > 0) {
        oauth2RedirectUrl = window.location.href.substring(0, query);
    } else {
        oauth2RedirectUrl = window.location.href;
    }

    if (!oauth2RedirectUrl.endsWith("/")) {
        oauth2RedirectUrl += "/";
    }
    oauth2RedirectUrl += "oauth-receiver.html";

    SwaggerUIBundle({
        dom_id: '#ui',
        url: "{:url}",
        filter: false,
        oauth2RedirectUrl: oauth2RedirectUrl,
    })
</script>

</body>
</html>
"#;

fn create_html(url: &str) -> String {
    SWAGGER_UI_TEMPLATE
        .replace("{:style}", SWAGGER_UI_CSS)
        .replace("{:script}", SWAGGER_UI_JS)
        .replace("{:url}", url)
}

pub fn create_endpoint(url: &str) -> impl Endpoint {
    let ui_html = create_html(url);
    poem::Route::new()
        .at("/", make_sync(move |_| Html(ui_html.clone())))
        .at(
            "/oauth-receiver.html",
            make_sync(move |_| Html(OAUTH_RECEIVER_HTML.to_string())),
        )
}
