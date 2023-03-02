use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;

use crate::{config::CONFIG, TASKS_TRANSACTION_PK, TASKS_TRANSACTION_SK};

pub async fn is_transaction_in_progress() -> Result<bool> {
    get_dynamodb_client()
        .await
        .get_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("PK", AttributeValue::S(TASKS_TRANSACTION_PK.to_owned()))
        .key("SK", AttributeValue::S(TASKS_TRANSACTION_SK.to_owned()))
        .send()
        .await
        .map(|result| result.item)
        .map_err(|e| Error::unknown(format!("Failed to get current transaction state: {e:?}")))
        .map(|item| item.is_some())
}
