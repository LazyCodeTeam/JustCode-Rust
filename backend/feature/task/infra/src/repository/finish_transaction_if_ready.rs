use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::get_dynamodb_client;

use crate::{config::CONFIG, TASKS_TRANSACTION_PK, TASKS_TRANSACTION_SK};

pub async fn finish_transaction_if_ready() -> Result<()> {
    get_dynamodb_client()
        .await
        .delete_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("PK", AttributeValue::S(TASKS_TRANSACTION_PK.to_owned()))
        .key("SK", AttributeValue::S(TASKS_TRANSACTION_SK.to_owned()))
        .condition_expression("processed_items_count = items_count")
        .send()
        .await
        .ok();

    Ok(())
}
