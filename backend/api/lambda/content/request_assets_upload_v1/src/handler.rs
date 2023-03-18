use common_api::{dto::PresignedUrlDto, lambda::into_response::IntoResponse, FromModel};
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use use_case::content::request_assets_upload::{
    request_assets_upload, RequestAssetsUploadRepository,
};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    let count = event
        .query_string_parameters()
        .first("count")
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(1);

    request_assets_upload(
        count,
        RequestAssetsUploadRepository {
            get_upload_url: bucket_infra::repository::get_upload_url,
        },
    )
    .await
    .map(|urls| {
        urls.into_iter()
            .map(PresignedUrlDto::from_model)
            .collect::<Vec<_>>()
    })
    .into_response(StatusCode::OK)
}
