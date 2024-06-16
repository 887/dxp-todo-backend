use poem::{session::SessionStorage, Request};
use poem_openapi::{auth::ApiKey, SecurityScheme};

use crate::{
    session::{api_session::ApiSession, api_session_container::ApiSessionContainer},
    state::State,
};

use tracing::error;

#[derive(SecurityScheme)]
#[oai(
    ty = "api_key",
    key_name = "X-API-Key",
    key_in = "query",
    checker = "api_checker"
)]
pub struct ApiKeySecurityScheme(pub ApiSessionContainer);

impl ApiKeySecurityScheme {
    pub fn session(&self) -> &ApiSession {
        &self.0.session
    }
    pub async fn update(&mut self) -> Result<(), poem::Error> {
        self.0.update().await
    }
}

async fn api_checker(req: &Request, api_key: ApiKey) -> Option<ApiSessionContainer> {
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

    let api_session = entries
        .map(ApiSession::new)
        .unwrap_or(ApiSession::default());

    Some(ApiSessionContainer::new(
        api_key,
        api_session,
        state.storage.clone(),
    ))
}
