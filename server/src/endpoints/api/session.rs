use std::collections::BTreeMap;

use poem::{http::StatusCode, Error};
use poem_openapi::{
    payload::{Json, PlainText},
    OpenApi,
};
use tracing::trace;

use serde_json::Value;

pub struct SessionApi;

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Session operations
    Session,
}

#[OpenApi]
impl SessionApi {
    /// Session
    #[oai(
        path = "/load_session",
        method = "get",
        tag = "Tags::Session",
        operation_id = "load_session"
    )]
    async fn load_session(&self) -> poem::Result<Json<BTreeMap<String, Value>>> {
        trace!("/load_session");
        // Ok(PlainText("Hello, World!".to_string()))
        Err(poem::Error::from(StatusCode::NOT_FOUND))
    }

    #[oai(
        path = "/update_session",
        method = "post",
        tag = "Tags::Session",
        operation_id = "update_session"
    )]
    async fn update_session(&self, entries: Json<BTreeMap<String, Value>>) -> poem::Result<()> {
        trace!("/update_session");
        Ok(())
    }

    #[oai(
        path = "/remove_session",
        method = "post",
        tag = "Tags::Session",
        operation_id = "remove_session"
    )]
    async fn remove_session(&self, entries: Json<BTreeMap<String, Value>>) -> poem::Result<()> {
        trace!("/remove_session");
        Ok(())
    }
}
