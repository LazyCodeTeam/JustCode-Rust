use common_api::lambda::into_response::IntoResponse;
use common_api::lambda::lambda_error::LambdaError;
use common_api::lambda::lambda_request_ext::LambdaRequestExt;
use common_api::lambda::register_handler::register_handler;
use content_dto::{MapFrom, MapInto, PersonalizedTaskDto};
use lambda_http::Error;
use lambda_http::{http::StatusCode, Body, Request, Response};
use use_case::content::get_tasks::{get_tasks, GetTasksRepo};

const SECTION_ID_PARAM_NAME: &str = "section_id";

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}
pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let section_id = event.required_path_parameter(SECTION_ID_PARAM_NAME)?;
    let user_id = event.user_id()?;
    get_tasks(
        section_id,
        user_id,
        GetTasksRepo {
            get_tasks: content_infra::repository::get_ordered_section_tasks,
            get_valid_historical_answers: content_infra::repository::get_first_valid_answers,
        },
    )
    .await
    .map(Vec::<PersonalizedTaskDto>::map_from)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
