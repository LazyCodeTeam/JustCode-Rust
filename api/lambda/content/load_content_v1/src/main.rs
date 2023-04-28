use common_api::lambda::{
    into_response::IntoEmptyRespone, lambda_error::LambdaError,
    lambda_request_ext::LambdaRequestExt, register_handler::register_handler,
};
use content_domain::model::expected_technology_data::ExpectedTechnologyData;
use content_dto::{ExpectedTechnologyDto, MapFrom, MapInto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::load_content::{load_tasks, LoadContentRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let expected_technologies: Vec<ExpectedTechnologyDto> = event.deserialized_body()?;
    let expected_technologies = Vec::<ExpectedTechnologyData>::map_from(expected_technologies);

    load_tasks(
        expected_technologies,
        LoadContentRepository {
            get_full_content: content_infra::repository::get_full_content,
            add_modifications_to_queue: content_infra::repository::add_modifications_to_queue,
            is_transaction_in_progress: content_infra::repository::is_transaction_in_progress,
            begin_transaction: content_infra::repository::begin_transaction,
            increase_queue_items_count: content_infra::repository::increase_queue_items_count,
        },
    )
    .await
    .map_err(MapInto::map_into)?
    .into_empty_response(StatusCode::ACCEPTED)
}
