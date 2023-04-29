use common_api::lambda::{
    into_response::IntoEmptyRespone, lambda_error::LambdaError,
    lambda_request_ext::LambdaRequestExt, register_handler::register_handler,
};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_domain::model::update_profile_params::UpdateProfileParams;
use profile_dto::{MapInto, UpdateProfileDto};
use profile_infra::repository;
use use_case::profile::update_profile::{update_profile, UpdateProfileRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let params: UpdateProfileDto = event.deserialized_body()?;
    let params: UpdateProfileParams = params.map_into();
    let user_id = event.user_id()?;
    update_profile(
        (user_id, params),
        UpdateProfileRepository {
            get_profile_by_id: repository::get_profile_by_id,
            update_profile: repository::update_profile,
        },
    )
    .await
    .into_empty_response(StatusCode::OK)
}
