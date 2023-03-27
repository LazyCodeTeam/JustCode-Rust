use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::content_asset::ContentAsset;

use crate::{config::CONFIG, content_asset_dto::ContentAssetDto, CONTENT_ASSET_PK};

pub async fn get_content_assets() -> Result<Vec<ContentAsset>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk")
        .expression_attribute_values(":pk", AttributeValue::S(CONTENT_ASSET_PK.to_owned()))
        .send()
        .await
        .parse::<ContentAssetDto>()
        .map(|o| o.into_iter().map(Into::into).collect())
}
