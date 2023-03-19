use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::content_asset_creation_data::ContentAssetCreationData;
use serde_dynamo::to_item;

use crate::{config::CONFIG, content_asset_dto::ContentAssetDto};

pub async fn save_content_asset(content_asset: ContentAssetCreationData) -> Result<()> {
    let item = to_item(ContentAssetDto::from(content_asset))
        .map_err(|e| Error::unknown(format!("Failed to serialize asset ({e:?})")))?;

    get_dynamodb_client()
        .await
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map_err(|e| Error::unknown(format!("Failed to save asset data: {e:?}")))
        .map(|_| ())
}
