use common_api::lambda::{
    into_response::IntoResponse, lambda_error::LambdaError, lambda_request_ext::LambdaRequestExt,
    register_handler::register_handler,
};
use content_dto::{MapInto, PublicTaskDto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_public_tasks::{get_public_tasks, GetPublicTasksRepo};

const SECTION_ID_PARAM_NAME: &str = "section_id";

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let section_id = event.required_path_parameter(SECTION_ID_PARAM_NAME)?;

    get_public_tasks(
        section_id,
        GetPublicTasksRepo {
            get_tasks: content_infra::repository::get_ordered_section_tasks,
        },
    )
    .await
    .map::<Vec<PublicTaskDto>, _>(MapInto::map_into)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
