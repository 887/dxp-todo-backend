use std::collections::BTreeMap;

use poem::{session::SessionStorage, Request};
use poem_openapi::{auth::ApiKey, SecurityScheme};

use crate::{session::ApiSession, state::State};

use tracing::error;

#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "query",
    checker = "api_checker"
)]
pub struct ApiKeySecurityScheme(pub ApiSession);

impl ApiKeySecurityScheme {
    pub fn session(&mut self) -> &mut ApiSession {
        &mut self.0
    }
}

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<ApiSession> {
    let api_key = api_key.key;
    let state = req.data::<State>()?;

    let entries_maybe = state.storage.load_session(&api_key).await;
    let entries = match entries_maybe {
        Ok(e) => e,
        Err(err) => {
            error!("Failed to load session {:?}", err);
            return None;
        }
    };

    let entries = entries.unwrap_or(BTreeMap::default());

    Some(ApiSession::new(api_key, state.storage.clone(), entries))
}
