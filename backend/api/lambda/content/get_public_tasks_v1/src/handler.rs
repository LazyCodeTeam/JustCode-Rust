use common_api::lambda::into_response::IntoResponse;
use common_domain::into_future::IntoFuture;
use content_dto::output::public_task_dto::PublicTaskDto;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use use_case::content::get_public_tasks::{get_public_tasks, GetPublicTasksRepo};

const SECTION_ID_PARAM_NAME: &str = "section_id";

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .path_parameters()
        .first(SECTION_ID_PARAM_NAME)
        .ok_or_else(|| common_domain::error::Error::unknown("Failed to get section_id from url"))
        .into_future()
        .and_then(|technology_id| {
            get_public_tasks(
                technology_id.to_owned(),
                GetPublicTasksRepo {
                    get_tasks: content_infra::repository::get_section_tasks,
                },
            )
        })
        .await
        .map(|sections| {
            sections
                .into_iter()
                .map(Into::into)
                .collect::<Vec<PublicTaskDto>>()
        })
        .into_response(StatusCode::OK)
}
