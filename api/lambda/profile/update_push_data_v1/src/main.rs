use common_api::lambda::{
    into_response::IntoEmptyRespone, lambda_error::LambdaError,
    lambda_request_ext::LambdaRequestExt, register_handler::register_handler,
};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_domain::model::push_data::PushData;
use profile_dto::{MapInto, PushDataDto};
use use_case::profile::set_push_data::{set_push_data, SetPushDataRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}
pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let dto: PushDataDto = event.deserialized_body()?;
    let push_data: PushData = dto.map_into();
    let user_id = event.user_id()?;

    set_push_data(
        (user_id, Some(push_data)),
        SetPushDataRepository {
            update_push_data: profile_infra::repository::update_push_data,
            remove_push_data: profile_infra::repository::remove_push_data,
        },
    )
    .await
    .map_err(MapInto::map_into)?
    .into_empty_response(StatusCode::OK)
}
