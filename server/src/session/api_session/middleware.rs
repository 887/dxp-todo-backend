use std::sync::Arc;

// use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
// use rand::{thread_rng, Rng};

use poem::{
    session::{Session, SessionStatus, SessionStorage},
    Endpoint, Middleware, Request, Result,
};

use super::{ApiSession, ApiSessionStatus};

/// Middleware for server-side session.
pub struct ApiSessionExtractor<T> {
    header: &'static str,
    storage: Arc<T>,
}

impl<T> ApiSessionExtractor<T> {
    /// Create a `ServerSession` middleware.
    pub fn new(storage: T, header: &'static str) -> Self {
        Self {
            header,
            storage: Arc::new(storage),
        }
    }
}

impl<T: SessionStorage, E: Endpoint> Middleware<E> for ApiSessionExtractor<T> {
    type Output = ApiSessionEndpoint<T, E>;

    fn transform(&self, ep: E) -> Self::Output {
        ApiSessionEndpoint {
            header: self.header,
            inner: ep,
            storage: self.storage.clone(),
        }
    }
}

// fn generate_session_id() -> String {
//     let random_bytes = thread_rng().gen::<[u8; 32]>();
//     URL_SAFE_NO_PAD.encode(random_bytes)
// }

/// Endpoint for `ServerSession` middleware.
pub struct ApiSessionEndpoint<T, E> {
    inner: E,
    header: &'static str,
    storage: Arc<T>,
}

impl<T, E> Endpoint for ApiSessionEndpoint<T, E>
where
    T: SessionStorage,
    E: Endpoint,
{
    type Output = E::Output;

    async fn call(&self, mut req: Request) -> Result<Self::Output> {
        let header_maybe = req.headers().get(self.header);
        let mut session_id = header_maybe.and_then(|hv| hv.to_str().ok());
        let session = match session_id {
            Some(id) => match self.storage.load_session(id).await? {
                Some(entries) => ApiSession::new(entries),
                None => {
                    session_id = None;
                    ApiSession::default()
                }
            },
            None => ApiSession::default(),
        };

        req.extensions_mut().insert(session.clone());
        let resp = self.inner.call(req).await?;

        // match session.status() {
        //     ApiSessionStatus::Changed => match session_id {
        //         Some(session_id) => {
        //             self.storage
        //                 .update_session(&session_id, &session.entries())
        //                 .await?;
        //         }
        //         None => {
        //             let session_id = generate_session_id();
        //             self.config.set_cookie_value(&cookie_jar, &session_id);
        //             self.storage
        //                 .update_session(&session_id, &session.entries(), self.config.ttl())
        //                 .await?;
        //         }
        //     },
        //     SessionStatus::Renewed => {
        //         if let Some(session_id) = session_id {
        //             self.storage.remove_session(&session_id).await?;
        //         }

        //         let session_id = generate_session_id();
        //         self.config.set_cookie_value(&cookie_jar, &session_id);
        //         self.storage
        //             .update_session(&session_id, &session.entries(), self.config.ttl())
        //             .await?;
        //     }
        //     SessionStatus::Purged => {
        //         if let Some(session_id) = session_id {
        //             self.storage.remove_session(&session_id).await?;
        //             self.config.remove_cookie(&cookie_jar);
        //         }
        //     }
        //     SessionStatus::Unchanged => {}
        // };

        Ok(resp)
    }
}
