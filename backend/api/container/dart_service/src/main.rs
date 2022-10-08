mod controller;

use actix_web::{web, App, HttpServer};
use code_infra::create_project::create_base_dart_project;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    create_base_dart_project().expect("Failed to create base dart project");

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api").service(
                web::scope("/v1").service(
                    web::scope("dart")
                        .service(controller::analyze_raw)
                        .service(controller::format),
                ),
            ),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
