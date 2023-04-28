use crate::config::CONFIG;
use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use snafu::ResultExt;

use crate::{PROFILE_ID_PREFIX, PROFILE_PRIMARY_KEY};

pub async fn update_profile_avatar<S, S2>(id: S, url: Option<S2>) -> Result<()>
where
    S: Into<String>,
    S2: Into<String>,
{
    let query = get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.dynamodb_table)
        .key(
            "SK",
            AttributeValue::S(format!("{PROFILE_ID_PREFIX}{}", id.into())),
        )
        .key("PK", AttributeValue::S(PROFILE_PRIMARY_KEY.to_owned()));

    let query = match url {
        Some(url) => query
            .update_expression("set avatar_url = :avatar_url")
            .expression_attribute_values(":avatar_url", AttributeValue::S(url.into())),
        None => query.update_expression("remove avatar_url"),
    };

    query
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to update profile avatar")
        .with_error_log()
}
