use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::content_asset::ContentAsset;
use serde_dynamo::from_items;

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
        .map_err(|e| Error::unknown(format!("Failed to get content assets: {e:?}")))
        .and_then(|r| {
            r.items
                .ok_or_else(|| {
                    Error::unknown("Failed to get content assets - option is empty".to_owned())
                })
                .and_then(|items| {
                    from_items::<_, ContentAssetDto>(items)
                        .map_err(|e| {
                            Error::unknown(format!("Failed to parse content assets: {e:?}"))
                        })
                        .map(|dtos| dtos.into_iter().map(Into::into).collect())
                })
        })
}
