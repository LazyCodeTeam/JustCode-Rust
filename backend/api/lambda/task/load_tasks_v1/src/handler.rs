use common_api::lambda::into_response::IntoResponse;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::task::load_tasks::{load_tasks, LoadTasksRepository};

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    load_tasks(LoadTasksRepository {})
        .await
        .into_empty_response(StatusCode::OK)
}
