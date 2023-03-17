use common_api::lambda::{from_request::FromRequest, into_response::IntoEmptyRespone};
use common_domain::into_future::IntoFuture;
use content_domain::model::expected_technology_data::ExpectedTechnologyData;
use content_dto::{ExpectedTechnologyDto, FromDto};
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::load_content::{load_tasks, LoadContentRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    Vec::<ExpectedTechnologyDto>::from_request(&event)
        .map(|data| {
            data.into_iter()
                .map(ExpectedTechnologyData::from_dto)
                .collect()
        })
        .into_future()
        .and_then(|data| {
            load_tasks(
                data,
                LoadContentRepository {
                    get_full_content: content_infra::repository::get_full_content,
                    add_modifications_to_queue: |_| async { Ok(()) },
                    is_transaction_in_progress: || async { Ok(false) },
                    begin_transaction: |_| async { Ok(()) },
                    increase_queue_items_count: |_| async { Ok(()) },
                },
            )
        })
        .await
        .into_empty_response(StatusCode::ACCEPTED)
}
