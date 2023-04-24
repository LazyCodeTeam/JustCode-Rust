use common_api::lambda::{into_response::IntoResponse, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use content_dto::{FromModel, PersonalizedTaskDto};
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use use_case::content::get_tasks::{get_tasks, GetTasksRepo};

const SECTION_ID_PARAM_NAME: &str = "section_id";

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .path_parameters()
        .first(SECTION_ID_PARAM_NAME)
        .ok_or_else(|| common_domain::error::Error::unknown("Failed to get section_id from url"))
        .and_then(
            |section_id| -> Result<(String, String), common_domain::error::Error> {
                Ok((section_id.to_owned(), event.get_user_id()?))
            },
        )
        .into_future()
        .and_then(|(section_id, user_id)| {
            get_tasks(
                section_id,
                user_id,
                GetTasksRepo {
                    get_tasks: content_infra::repository::get_ordered_section_tasks,
                    get_valid_historical_answers:
                        content_infra::repository::get_first_valid_answers,
                },
            )
        })
        .await
        .map(Vec::<PersonalizedTaskDto>::from_model)
        .into_response(StatusCode::OK)
}
