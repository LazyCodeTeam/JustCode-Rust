use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use content_domain::model::content_asset_creation_data::ContentAssetCreationData;
use serde_dynamo::to_item;
use snafu::ResultExt;

use crate::{config::CONFIG, content_asset_dto::ContentAssetDto, MapFrom};

pub async fn save_content_asset(content_asset: ContentAssetCreationData) -> Result<()> {
    let item = to_item(ContentAssetDto::map_from(content_asset))
        .whatever_context("Failed to serialize asset")
        .with_error_log()?;

    get_dynamodb_client()
        .await
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to save asset")
        .with_error_log()
}
