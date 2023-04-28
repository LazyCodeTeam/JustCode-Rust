use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use snafu::ResultExt;

use crate::{config::CONFIG, TASKS_TRANSACTION_PK, TASKS_TRANSACTION_SK};

pub async fn is_transaction_in_progress() -> Result<bool> {
    get_dynamodb_client()
        .await
        .get_item()
        .table_name(&CONFIG.content_dynamodb_table)
        .key("PK", AttributeValue::S(TASKS_TRANSACTION_PK.to_owned()))
        .key("SK", AttributeValue::S(TASKS_TRANSACTION_SK.to_owned()))
        .send()
        .await
        .map(|result| result.item)
        .map(|item| item.is_some())
        .whatever_context("Failed to get current transaction state")
        .with_error_log()
}
