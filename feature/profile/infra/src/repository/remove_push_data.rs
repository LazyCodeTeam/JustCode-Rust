use crate::config::CONFIG;
use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use snafu::ResultExt;

use crate::{PROFILE_ID_PREFIX, PROFILE_PRIMARY_KEY};

pub async fn remove_push_data(id: &str) -> Result<()> {
    get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("SK", AttributeValue::S(format!("{PROFILE_ID_PREFIX}{id}")))
        .key("PK", AttributeValue::S(PROFILE_PRIMARY_KEY.to_owned()))
        .update_expression("remove push_token, platform")
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to remove push data")
        .with_error_log()
}
