use common_api::lambda::{
    from_request::FromRequest, into_response::IntoResponse, validate::Validate,
};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use task_domain::model::expected_technology_data::ExpectedTechnologyData;
use use_case::task::load_tasks::{load_tasks, LoadTasksRepository};

use crate::dto::technology_dto::TechnologyDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    Vec::<TechnologyDto>::from_request(&event)
        .and_then(Validate::validate)
        .map(|data| data.into_iter().map(ExpectedTechnologyData::from).collect())
        .into_future()
        .and_then(|data| load_tasks(data, LoadTasksRepository {}))
        .await
        .into_empty_response(StatusCode::OK)
}
