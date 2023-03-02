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
        .and_then(|data| {
            load_tasks(
                data,
                LoadTasksRepository {
                    get_raw_tasks_tree: task_infra::repository::get_raw_tasks_tree,
                    add_modifications_to_queue: task_infra::repository::add_modifications_to_queue,
                    is_transaction_in_progress: task_infra::repository::is_transaction_in_progress,
                    begin_transaction: task_infra::repository::begin_transaction,
                    increase_queue_items_count: task_infra::repository::increase_queue_items_count,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::OK)
}
