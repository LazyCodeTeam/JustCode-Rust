use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::{config::CONFIG, dynamodb_client::get_dynamodb_client};

use crate::{PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

pub async fn remove_push_data(id: &str) -> Result<()> {
    get_dynamodb_client()
        .await
        .update_item()
        .table_name(&CONFIG.dynamodb_table)
        .key(
            "PK",
            AttributeValue::S(format!("{}{}", PROFILE_ID_PREFIX, id)),
        )
        .key("SK", AttributeValue::S(PROFILE_SORT_KEY.to_owned()))
        .update_expression("remove push_token, platform")
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("Failed to remove push data: {:?}", e)))
}
