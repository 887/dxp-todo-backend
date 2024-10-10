// This should always be the last layer in the middleware stack.

use axum::{extract::Request, response::Response};
use std::task::{Context, Poll};
use tower::{Layer, Service};
use tracing::error;

type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[derive(Clone)]
pub struct ErrorLayer {}

impl<S> Layer<S> for ErrorLayer {
    type Service = ErrorMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ErrorMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct ErrorMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for ErrorMiddleware<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Error: std::fmt::Debug,
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
        Box::pin(async move {
            match future.await {
                Ok(response) => Ok(response),
                Err(err) => {
                    error!(
                        "Error caught: \n\
                        {:?}",
                        err
                    );
                    Err(err)
                }
            }
        })
    }
}
