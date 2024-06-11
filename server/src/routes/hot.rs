use poem::{handler, middleware::AddData, web::Data, EndpointExt, IntoResponse, Route};
use poem_openapi::payload::PlainText;

#[derive(Debug, Clone)]
struct HotVersion {
    pub data: i64,
}

pub fn get_route() -> Route {
    let rn = chrono::Utc::now().timestamp();
    let hot_version = HotVersion { data: rn };

    Route::new().at(
        "/",
        Route::new()
            .nest("", loaded_version)
            .with(AddData::new(hot_version)),
    )
}

#[handler]
pub fn loaded_version(Data(hot_version): Data<&HotVersion>) -> impl IntoResponse {
    PlainText(hot_version.data.to_string())
}
