use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};
use profile_domain::model::{platform::Platform, push_data::PushData};

use crate::{PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

pub async fn update_push_data(id: &str, data: &PushData) -> Result<()> {
    let platform = match data.platform {
        Platform::Android => "ANDROID",
        Platform::Ios => "IOS",
        Platform::Unknown => return Err(Error::unknown("Unknown platform".to_owned())),
    };

    get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("PK", AttributeValue::S(format!("{PROFILE_ID_PREFIX}{id}")))
        .key("SK", AttributeValue::S(PROFILE_SORT_KEY.to_owned()))
        .update_expression("set push_token = :push_token, platform = :platform")
        .expression_attribute_values(":push_token", AttributeValue::S(data.token.clone()))
        .expression_attribute_values(":platform", AttributeValue::S(platform.to_owned()))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("Failed to set push data: {e:?}")))
}
