// This should always be the last layer in the middleware stack.
// Otherwise trace logging will not be complete.

use axum::{extract::Request, response::Response};
use std::task::{Context, Poll};
use tower::{Layer, Service};

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[derive(Clone)]
pub struct TracingLayer {
    pub log_dispatcher: dxp_logging::LogDispatcher,
}

impl<S> Layer<S> for TracingLayer {
    type Service = TracingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        let log_dispatcher = self.log_dispatcher.clone();
        TracingMiddleware {
            inner,
            log_dispatcher,
        }
    }
}

#[derive(Clone)]
pub struct TracingMiddleware<S> {
    inner: S,
    log_dispatcher: dxp_logging::LogDispatcher,
}

impl<S> Service<Request> for TracingMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let future = self.inner.call(request);
        let log_dispatcher = self.log_dispatcher.clone();
        Box::pin(async move {
            let log_guard = dxp_logging::set_thread_default_dispatcher(&log_dispatcher);
            let response: Response = future.await?;
            drop(log_guard);
            Ok(response)
        })
    }
}
