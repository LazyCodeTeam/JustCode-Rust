use bucket_infra::consts::PROFILE_AVATARS_PREFIX;
use common_api::{
    dto::PresignedUrlDto,
    lambda::{into_response::IntoResponse, user_context::UserContext},
    FromModel,
};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::profile::request_avatar_upload::{
    request_avatar_upload, RequestAvatarUploadRepository,
};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .into_future()
        .and_then(|id| {
            request_avatar_upload(
                id,
                RequestAvatarUploadRepository {
                    get_profile_by_id: profile_infra::repository::get_profile_by_id,
                    get_avatar_upload_url: |id, valid_for| {
                        bucket_infra::repository::get_upload_url(
                            PROFILE_AVATARS_PREFIX,
                            Some(id),
                            valid_for,
                        )
                    },
                },
            )
        })
        .await
        .map(PresignedUrlDto::from_model)
        .into_response(StatusCode::OK)
}
