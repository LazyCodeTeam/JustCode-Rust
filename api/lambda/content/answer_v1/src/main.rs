use common_api::lambda::{
    into_response::IntoResponse, lambda_error::LambdaError, lambda_request_ext::LambdaRequestExt,
    register_handler::register_handler,
};
use content_domain::model::answer::Answer;
use content_dto::{AnswerValidationResultDto, MapFrom, MapInto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::answer::{answer as try_answer, AnswerRepository};

const TASK_ID_PATH_PARAM: &str = "task_id";

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

async fn handle_request(request: Request) -> Result<Response<Body>, LambdaError> {
    let answer_dto = request.deserialized_body()?;
    let task_id = request.required_path_parameter(TASK_ID_PATH_PARAM)?;
    let user_id = request.user_id()?;
    let answer = Answer::map_from((task_id, answer_dto));

    try_answer(
        user_id,
        answer,
        AnswerRepository {
            get_task: content_infra::repository::get_task_by_id,
            get_previous_answers: content_infra::repository::get_previous_answers_for_task,
            save_answer: content_infra::repository::save_answer,
        },
    )
    .await
    .map(AnswerValidationResultDto::map_from)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
