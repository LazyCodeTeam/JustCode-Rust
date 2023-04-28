use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use snafu::ResultExt;

use crate::{config::CONFIG, PROFILE_ID_PREFIX, PROFILE_PRIMARY_KEY};

pub async fn delete_profile_by_id(id: impl Into<String>) -> Result<()> {
    get_dynamodb_client()
        .await
        .delete_item()
        .table_name(&CONFIG.dynamodb_table)
        .key(
            "SK",
            AttributeValue::S(format!("{PROFILE_ID_PREFIX}{}", id.into())),
        )
        .key("PK", AttributeValue::S(PROFILE_PRIMARY_KEY.to_owned()))
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to delete profile by id")
        .with_error_log()
}
