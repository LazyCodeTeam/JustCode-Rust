mod controller;
pub mod dto;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::*;

macro_rules! new_lang {
    ($lang:literal, $router:ident) => {
        paste::paste! {
            code_infra::repository::[<$lang>]::create_base_project().await
                .expect(&format!("Failed to create base {} project", $lang));

            let [<$lang _router>] = Router::new()
                .route("/analyze/raw", post(controller::[<$lang>]::analyze_raw))
                .route("/analyze", post(controller::[<$lang>]::analyze))
                .route("/format", post(controller::[<$lang>]::format))
                .route("/build", post(controller::[<$lang>]::build))
                .route("/version", get(controller::[<$lang>]::get_version));
            let $router = $router.nest(&format!("/api/v1/{}", $lang), [<$lang _router>]);
        }
    };
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let router = Router::new().layer(TraceLayer::new_for_http());

    #[cfg(feature = "dart")]
    new_lang!("dart", router);

    #[cfg(feature = "flutter")]
    new_lang!("flutter", router);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to serve http server")
}
