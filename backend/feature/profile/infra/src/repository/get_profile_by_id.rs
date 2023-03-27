use crate::config::CONFIG;
use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, GetItemOutputExt};
use profile_domain::model::profile::Profile;

use crate::{dto::profile_dto::ProfileDto, PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

pub async fn get_profile_by_id(id: &str) -> Result<Option<Profile>> {
    get_dynamodb_client()
        .await
        .get_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("PK", AttributeValue::S(format!("{PROFILE_ID_PREFIX}{id}")))
        .key("SK", AttributeValue::S(PROFILE_SORT_KEY.to_owned()))
        .send()
        .await
        .parse::<ProfileDto>()
        .map(|o| o.map(|p| p.into()))
}
