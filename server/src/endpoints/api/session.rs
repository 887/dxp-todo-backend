use std::collections::BTreeMap;

use poem::{session::SessionStorage, web::Data};
use poem_openapi::{param::Query, payload::Json, Object, OpenApi};
use tracing::trace;

use serde_json::Value;

use crate::endpoints::session::SessionStorageObject;

pub struct SessionApi;

#[derive(poem_openapi::Tags)]
enum Tags {
    /// Session operations
    Session,
}

#[derive(Object, Debug, PartialEq)]
pub struct UpdateSessionValue {
    pub entries: BTreeMap<String, Value>,
    pub expires: Option<u64>,
}

#[derive(Object, Debug, PartialEq)]
pub struct LoadSessionValue {
    pub exists: bool,
    pub entries: Option<BTreeMap<String, Value>>,
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
        session_id: Query<String>,
    ) -> poem::Result<Json<LoadSessionValue>> {
        trace!("/load_session");
        let entries = session.load_session(&session_id).await?;
        let exists = entries != None;
        Ok(Json(LoadSessionValue { exists, entries }))
    }

    #[oai(
        path = "/update_session",
        method = "put",
        tag = "Tags::Session",
        operation_id = "update_session"
    )]
    async fn update_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: Query<String>,
        value: Json<UpdateSessionValue>,
    ) -> poem::Result<()> {
        trace!("/update_session");
        let entries = value.0.entries;
        let expires = value.0.expires;

        session
            .update_session(
                &session_id,
                &entries,
                expires.map(std::time::Duration::from_secs),
            )
            .await
    }

    #[oai(
        path = "/remove_session",
        method = "delete",
        tag = "Tags::Session",
        operation_id = "remove_session"
    )]
    async fn remove_session(
        &self,
        session: Data<&SessionStorageObject>,
        session_id: Query<String>,
    ) -> poem::Result<()> {
        trace!("/remove_session");
        session.remove_session(&session_id).await
    }
}
