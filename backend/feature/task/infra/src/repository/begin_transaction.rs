use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use serde_dynamo::to_item;

use crate::{config::CONFIG, dto::tasks_transaction_dto::TasksTransactionDto};

pub async fn begin_transaction(items_count: u64) -> Result<()> {
    let item = to_item(TasksTransactionDto::new(items_count))
        .map_err(|e| Error::unknown(format!("Failed to serialize transaction ({e:?})")))?;

    get_dynamodb_client()
        .await
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map_err(|e| Error::unknown(format!("Failed to begin transaction: {e:?}")))
        .map(|_| ())
}
