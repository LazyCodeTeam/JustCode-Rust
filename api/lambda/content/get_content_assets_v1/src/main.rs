use common_api::lambda::{
    into_response::IntoResponse, lambda_error::LambdaError, register_handler::register_handler,
};
use content_dto::{ContentAssetDto, MapInto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_content_assets::{get_content_assets, GetContentAssetsRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

async fn handle_request(_event: Request) -> Result<Response<Body>, LambdaError> {
    get_content_assets(GetContentAssetsRepository {
        get_assets: content_infra::repository::get_content_assets,
    })
    .await
    .map_err(MapInto::map_into)
    .map::<Vec<ContentAssetDto>, _>(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
