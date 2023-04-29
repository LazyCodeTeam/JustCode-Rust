use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use serde_dynamo::to_item;
use snafu::ResultExt;

use crate::{config::CONFIG, dto::tasks_transaction_dto::TasksTransactionDto};

pub async fn begin_transaction(items_count: u64) -> Result<()> {
    let item = to_item(TasksTransactionDto::new(items_count))
        .whatever_context("Failed to serialize transaction")
        .with_error_log()?;

    get_dynamodb_client()
        .await
        .put_item()
        .table_name(&CONFIG.content_dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to begin transaction")
        .with_error_log()
}
