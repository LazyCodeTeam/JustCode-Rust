use crate::{config::CONFIG, MapFrom};
use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use profile_domain::model::push_data::PushData;
use snafu::ResultExt;

use crate::{dto::platform_dto::PlatformDto, PROFILE_ID_PREFIX, PROFILE_PRIMARY_KEY};

pub async fn update_push_data(id: &str, data: &PushData) -> Result<()> {
    get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("SK", AttributeValue::S(format!("{PROFILE_ID_PREFIX}{id}")))
        .key("PK", AttributeValue::S(PROFILE_PRIMARY_KEY.to_owned()))
        .update_expression("set push_token = :push_token, platform = :platform")
        .expression_attribute_values(":push_token", AttributeValue::S(data.token.clone()))
        .expression_attribute_values(
            ":platform",
            serde_dynamo::to_attribute_value(PlatformDto::map_from(data.platform))
                .whatever_context("Failed to serialize platform")
                .with_error_log()?,
        )
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to update push data")
        .with_error_log()
}
