use crate::{config::CONFIG, IntoModel};
use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, GetItemOutputExt};
use profile_domain::model::profile::Profile;

use crate::{dto::profile_dto::ProfileDto, PROFILE_ID_PREFIX, PROFILE_PRIMARY_KEY};

pub async fn get_profile_by_id(id: &str) -> Result<Option<Profile>> {
    get_dynamodb_client()
        .await
        .get_item()
        .table_name(&CONFIG.dynamodb_table)
        .key("SK", AttributeValue::S(format!("{PROFILE_ID_PREFIX}{id}")))
        .key("PK", AttributeValue::S(PROFILE_PRIMARY_KEY.to_owned()))
        .send()
        .await
        .parse::<ProfileDto>()
        .map(|o| o.map(IntoModel::into_model))
}
