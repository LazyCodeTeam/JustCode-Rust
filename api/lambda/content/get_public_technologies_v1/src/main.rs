use common_api::lambda::{
    into_response::IntoResponse, lambda_error::LambdaError, register_handler::register_handler,
};
use content_dto::{MapInto, TechnologyDto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_public_technologies::{
    get_public_technologies, GetPublicTechnologiesRepo,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(_event: Request) -> Result<Response<Body>, LambdaError> {
    get_public_technologies(GetPublicTechnologiesRepo {
        get_technologies: content_infra::repository::get_ordered_technologies,
    })
    .await
    .map::<Vec<TechnologyDto>, _>(MapInto::map_into)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
