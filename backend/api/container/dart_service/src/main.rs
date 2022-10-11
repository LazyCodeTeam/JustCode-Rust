mod controller;

use axum::{routing::post, Router};
use code_infra::create_project::create_base_dart_project;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    create_base_dart_project().expect("Failed to create base dart project");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let router = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/api/v1/dart/analyze", post(controller::analyze_raw))
        .route("/api/v1/dart/format", post(controller::format));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to serve http server")
}
