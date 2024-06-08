use std::collections::BTreeMap;

use poem::{http::StatusCode, session::SessionStorage, web::Data};
use poem_openapi::{payload::Json, OpenApi};
use tracing::trace;

use serde_json::Value;

use crate::endpoints::session::SessionStorageObject;

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
    async fn load_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: String,
    ) -> poem::Result<Json<Option<BTreeMap<String, Value>>>> {
        trace!("/load_session");
        Ok(Json(session.load_session(&session_id).await?))
    }

    #[oai(
        path = "/update_session",
        method = "post",
        tag = "Tags::Session",
        operation_id = "update_session"
    )]
    async fn update_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: String,
        entries: Json<BTreeMap<String, Value>>,
        expires: Json<Option<u64>>,
    ) -> poem::Result<()> {
        trace!("/update_session");
        let expires = expires.0;
        Ok(session
            .update_session(
                &session_id,
                &entries,
                expires.map(|t| std::time::Duration::from_millis(t)),
            )
            .await?)
    }

    #[oai(
        path = "/remove_session",
        method = "post",
        tag = "Tags::Session",
        operation_id = "remove_session"
    )]
    async fn remove_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: String,
    ) -> poem::Result<()> {
        trace!("/remove_session");
        Ok(session.remove_session(&session_id).await?)
    }
}
