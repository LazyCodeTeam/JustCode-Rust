use common_api::lambda::{into_response::IntoResponse, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::profile::request_avatar_upload::{
    request_avatar_upload, RequestAvatarUploadRepository,
};

use crate::dto::presigned_url::PresignedUrlDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .into_future()
        .and_then(|id| {
            request_avatar_upload(
                id,
                RequestAvatarUploadRepository {
                    get_profile_by_id: profile_infra::repository::get_profile_by_id,
                    get_avatar_upload_url: bucket_infra::repository::get_upload_url,
                },
            )
        })
        .await
        .map(PresignedUrlDto::from)
        .into_response(StatusCode::OK)
}
