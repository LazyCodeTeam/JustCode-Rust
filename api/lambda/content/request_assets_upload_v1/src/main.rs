use bucket_infra::consts::CONTENT_ASSETS_PREFIX;
use common_api::{
    lambda::{
        into_response::IntoResponse, lambda_error::LambdaError,
        lambda_request_ext::LambdaRequestExt, register_handler::register_handler,
    },
    MapFrom,
};
use content_dto::PresignedUrlDto;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::request_assets_upload::{
    request_assets_upload, RequestAssetsUploadRepository,
};

const COUNT_QUERY_PARAM: &str = "count";

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let count = event
        .query_parameter(COUNT_QUERY_PARAM)
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(1);

    request_assets_upload(
        count,
        RequestAssetsUploadRepository {
            get_upload_url: |valid_for| {
                bucket_infra::repository::get_upload_url::<_, String>(
                    CONTENT_ASSETS_PREFIX,
                    None,
                    valid_for,
                )
            },
        },
    )
    .await
    .map(Vec::<PresignedUrlDto>::map_from)
    .map_err(content_dto::MapInto::map_into)?
    .into_response(StatusCode::OK)
}
