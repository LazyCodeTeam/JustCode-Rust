use aws_lambda_events::{
    serde_json,
    sqs::{SqsBatchResponse, SqsEvent},
};
use common_domain::into_future::IntoFuture;
use content_domain::model::modification::Modification;
use content_infra::{modification_dto::ModificationDto, repository};
use futures::TryFutureExt;
use lambda_runtime::{Error, LambdaEvent};
use use_case::content::on_modification_batch::{
    on_modification_batch, OnModificationBatchRepository,
};

pub async fn handle_event(event: LambdaEvent<SqsEvent>) -> Result<SqsBatchResponse, Error> {
    event
        .payload
        .records
        .into_iter()
        .map(|record| {
            record
                .body
                .ok_or_else(|| common_domain::error::Error::unknown("Empty event body"))
                .and_then(|body| {
                    serde_json::from_str::<ModificationDto>(&body).map_err(|e| {
                        common_domain::error::Error::unknown(format!("Invalid event body: {}", e))
                    })
                })
                .map(Into::into)
        })
        .collect::<common_domain::error::Result<Vec<Modification>>>()
        .into_future()
        .and_then(|changes| {
            on_modification_batch(
                changes,
                OnModificationBatchRepository {
                    is_transaction_in_progress: repository::is_transaction_in_progress,
                    increment_transaction_counter: repository::increment_transaction_counter,
                    finish_transaction_if_ready: repository::finish_transaction_if_ready,
                    write_modifications: repository::write_modifications,
                },
            )
        })
        .await
        .map_err(Box::new)?;

    Ok(SqsBatchResponse::default())
}
