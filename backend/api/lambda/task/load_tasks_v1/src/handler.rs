use common_api::lambda::{from_request::FromRequest, into_response::IntoResponse};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use git_domain::model::git_hook_event::GitHookEvent;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::task::load_tasks::{load_tasks, LoadTasksRepository};

use crate::dto::push_event_dto::PushEventDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    let event_type = event
        .headers()
        .get("x-github-event")
        .and_then(|event| event.to_str().ok());

    match event_type {
        Some("push") => PushEventDto::from_request(&event).map(GitHookEvent::from),
        _ => Ok(GitHookEvent::Unknown),
    }
    .into_future()
    .and_then(|event| load_tasks(event, LoadTasksRepository {}))
    .await
    .into_empty_response(StatusCode::OK)
}
