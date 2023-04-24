use common_api::lambda::into_response::IntoResponse;
use content_dto::{ContentAssetDto, IntoDto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_content_assets::{get_content_assets, GetContentAssetsRepository};

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    get_content_assets(GetContentAssetsRepository {
        get_assets: content_infra::repository::get_content_assets,
    })
    .await
    .map::<Vec<ContentAssetDto>, _>(IntoDto::into_dto)
    .into_response(StatusCode::OK)
}
