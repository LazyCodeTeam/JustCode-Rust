use common_api::lambda::{
    into_response::IntoResponse, lambda_error::LambdaError, lambda_request_ext::LambdaRequestExt,
    register_handler::register_handler,
};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_dto::{MapFrom, MapInto, ProfileDto};
use profile_infra::repository;
use use_case::profile::get_profile_by_id::{get_profile_by_id, GetProfileByIdRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let user_id = event.user_id()?;

    get_profile_by_id(
        user_id,
        GetProfileByIdRepository {
            get_profile_by_id: repository::get_profile_by_id,
        },
    )
    .await
    .map(ProfileDto::map_from)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
