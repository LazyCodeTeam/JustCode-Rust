use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use snafu::ResultExt;

use crate::{config::CONFIG, TASKS_TRANSACTION_PK, TASKS_TRANSACTION_SK};

pub async fn increment_transaction_counter(count: u64) -> Result<()> {
    get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.content_dynamodb_table)
        .key("PK", AttributeValue::S(TASKS_TRANSACTION_PK.to_owned()))
        .key("SK", AttributeValue::S(TASKS_TRANSACTION_SK.to_owned()))
        .update_expression("set #processed_items_count = #processed_items_count + :count_to_add")
        .expression_attribute_names("#processed_items_count", "processed_items_count")
        .expression_attribute_values(":count_to_add", AttributeValue::N(count.to_string()))
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to increment transaction counter")
        .with_error_log()
}
