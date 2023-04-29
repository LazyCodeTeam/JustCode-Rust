use bucket_infra::consts::PROFILE_AVATARS_PREFIX;
use common_api::{
    dto::PresignedUrlDto,
    lambda::{
        into_response::IntoResponse, lambda_error::LambdaError,
        lambda_request_ext::LambdaRequestExt, register_handler::register_handler,
    },
    MapFrom,
};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_dto::MapInto;
use use_case::profile::request_avatar_upload::{
    request_avatar_upload, RequestAvatarUploadRepository,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let user_id = event.user_id()?;

    request_avatar_upload(
        user_id,
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
    .await
    .map(PresignedUrlDto::map_from)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
