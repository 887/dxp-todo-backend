use poem_openapi::{
    payload::{Json, PlainText},
    Object, OpenApi,
};
use serde::{Deserialize, Serialize};
use tracing::trace;

pub struct TestApi;

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Test operations
    Test,
}

#[derive(Clone, Debug, Deserialize, Serialize, Object)]
pub struct Test {
    pub test: String,
}

#[OpenApi]
impl TestApi {
    #[oai(
        path = "/test",
        method = "put",
        tag = "Tags::Test",
        operation_id = "test"
    )]
    async fn test(&self, test: Json<Test>) -> PlainText<String> {
        trace!("/test");
        let t = test.0.test;
        PlainText(format!("test:{}", t))
    }
}
