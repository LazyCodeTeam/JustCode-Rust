use common_api::lambda::{
    from_request::FromRequest, into_response::IntoResponse, user_context::UserContext,
};
use common_domain::into_future::IntoFuture;
use content_domain::model::answer::Answer;
use content_dto::{AnswerDto, AnswerValidationResultDto, FromDto, FromModel};
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use use_case::content::answer::{answer as try_answer, AnswerRepository};

const TASK_ID_PATH_PARAM: &str = "task_id";

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    AnswerDto::from_request(&event)
        .and_then(|dto| {
            event
                .path_parameters()
                .first(TASK_ID_PATH_PARAM)
                .ok_or_else(|| {
                    common_domain::error::Error::unknown("No task id in path param".to_string())
                })
                .map(|task_id| (task_id.to_owned(), dto))
        })
        .map(Answer::from_dto)
        .and_then(|answer| -> common_domain::error::Result<(String, Answer)> {
            Ok((event.get_user_id()?, answer))
        })
        .into_future()
        .and_then(|(user_id, answer)| {
            try_answer(
                user_id,
                answer,
                AnswerRepository {
                    get_task: content_infra::repository::get_task_by_id,
                    get_previous_answers: content_infra::repository::get_previous_answers_for_task,
                    save_answer: content_infra::repository::save_answer,
                },
            )
        })
        .await
        .map(AnswerValidationResultDto::from_model)
        .into_response(StatusCode::ACCEPTED)
}
