mod controller;

use std::net::SocketAddr;

use axum::{routing::post, Router};
use code_infra::create_project::create_base_dart_project;

#[tokio::main]
async fn main() {
    create_base_dart_project().expect("Failed to create base dart project");

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let router = Router::new()
        .route("/api/v1/dart/analyze", post(controller::analyze_raw))
        .route("/api/v1/dart/format", post(controller::format));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to serve http server")
}
