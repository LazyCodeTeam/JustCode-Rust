use common_api::lambda::into_response::IntoEmptyRespone;
use common_api::lambda::lambda_error::LambdaError;
use common_api::lambda::lambda_request_ext::LambdaRequestExt;
use common_api::lambda::register_handler::register_handler;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_dto::MapInto;
use use_case::profile::delete_profile::{delete_profile, DeleteProfileRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let user_id = event.user_id()?;
    let username = event.username()?;
    delete_profile(DeleteProfileRepository {
        delete_current_profile: || profile_infra::repository::delete_profile_by_id(user_id),
        delete_current_user: || user_infra::repository::delete_user_by_username(username),
    })
    .await
    .map_err(MapInto::map_into)?
    .into_empty_response(StatusCode::OK)
}
