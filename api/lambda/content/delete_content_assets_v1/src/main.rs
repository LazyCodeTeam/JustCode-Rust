use bucket_infra::consts::CONTENT_ASSETS_PREFIX;
use common_api::lambda::{
    into_response::IntoEmptyRespone, lambda_error::LambdaError,
    lambda_request_ext::LambdaRequestExt, register_handler::register_handler,
};
use content_dto::MapInto;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::delete_content_assets::{
    delete_content_assets, DeleteContentAssetsRepository,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let ids = event.deserialized_body()?;

    delete_content_assets(
        ids,
        DeleteContentAssetsRepository {
            delete_asset_object: |id| {
                bucket_infra::repository::delete_s3_object(format!("{CONTENT_ASSETS_PREFIX}{id}"))
            },
            delete_assets_data: content_infra::repository::delete_assets_data,
        },
    )
    .await
    .map_err(MapInto::map_into)?
    .into_empty_response(StatusCode::OK)
}
