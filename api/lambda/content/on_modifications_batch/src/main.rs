use aws_lambda_events::{
    serde_json,
    sqs::{SqsBatchResponse, SqsEvent},
};
use common_api::lambda::register_internal_handler::register_internal_handler;
use content_domain::model::modification::Modification;
use content_infra::{modification_dto::ModificationDto, repository, MapInto};
use lambda_runtime::{Error, LambdaEvent};
use use_case::content::on_modification_batch::{
    on_modification_batch, OnModificationBatchRepository,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_internal_handler(handle_event).await
}

pub async fn handle_event(event: LambdaEvent<SqsEvent>) -> Result<SqsBatchResponse, Error> {
    let modifications = event
        .payload
        .records
        .into_iter()
        .flat_map(|record| record.body)
        .map(|body| serde_json::from_str::<ModificationDto>(&body).map(MapInto::map_into))
        .collect::<Result<Vec<Modification>, _>>()?;

    on_modification_batch(
        modifications,
        OnModificationBatchRepository {
            is_transaction_in_progress: repository::is_transaction_in_progress,
            increment_transaction_counter: repository::increment_transaction_counter,
            finish_transaction_if_ready: repository::finish_transaction_if_ready,
            write_modifications: repository::write_modifications,
        },
    )
    .await?;

    Ok(SqsBatchResponse::default())
}
