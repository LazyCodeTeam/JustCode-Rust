use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;

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
        .map_err(|e| Error::unknown(format!("Failed to delete profile: {e:?}")))
}
