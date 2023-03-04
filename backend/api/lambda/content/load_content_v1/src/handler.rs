use common_api::lambda::{
    from_request::FromRequest, into_response::IntoEmptyRespone, validate::Validate,
};
use common_domain::into_future::IntoFuture;
use content_domain::model::expected_technology_data::ExpectedTechnologyData;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::load_content::{load_tasks, LoadContentRepository};

use crate::dto::technology_dto::TechnologyDto;

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    Vec::<TechnologyDto>::from_request(&event)
        .and_then(Validate::validate)
        .map(|data| data.into_iter().map(ExpectedTechnologyData::from).collect())
        .into_future()
        .and_then(|data| {
            load_tasks(
                data,
                LoadContentRepository {
                    get_full_content: content_infra::repository::get_full_content,
                    add_modifications_to_queue:
                        content_infra::repository::add_modifications_to_queue,
                    is_transaction_in_progress:
                        content_infra::repository::is_transaction_in_progress,
                    begin_transaction: content_infra::repository::begin_transaction,
                    increase_queue_items_count:
                        content_infra::repository::increase_queue_items_count,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::ACCEPTED)
}
