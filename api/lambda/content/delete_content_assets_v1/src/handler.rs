use bucket_infra::consts::CONTENT_ASSETS_PREFIX;
use common_api::lambda::{from_request::FromRequest, into_response::IntoEmptyRespone};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::delete_content_assets::{
    delete_content_assets, DeleteContentAssetsRepository,
};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    Vec::<String>::from_request(&event)
        .into_future()
        .and_then(|ids| {
            delete_content_assets(
                ids,
                DeleteContentAssetsRepository {
                    delete_asset_object: |id| {
                        bucket_infra::repository::delete_s3_object(format!(
                            "{CONTENT_ASSETS_PREFIX}{id}"
                        ))
                    },
                    delete_assets_data: content_infra::repository::delete_assets_data,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::OK)
}
