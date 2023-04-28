use std::{convert::Infallible, future::Future};

use lambda_http::{run, tower::ServiceBuilder, Body, Error, Request, Response};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::MapInto;

use super::lambda_error::LambdaError;

pub async fn register_handler<FUT>(handler: fn(Request) -> FUT) -> Result<(), Error>
where
    FUT: Future<Output = Result<Response<Body>, LambdaError>> + Send,
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .without_time()
        .init();

    let layer = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    let service = ServiceBuilder::new()
        .layer(layer)
        .service_fn(|event| async move {
            let response = handler(event).await;
            let response = match response {
                Ok(response) => response,
                Err(error) => error.map_into(),
            };
            Ok::<Response<Body>, Infallible>(response)
        });

    run(service).await
}
