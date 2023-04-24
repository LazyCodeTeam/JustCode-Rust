use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, WriteRequest};
use common_domain::error::Result;
use common_infra::{dynamodb_client::get_dynamodb_client, DYNAMODB_MAX_BATCH_SIZE};
use futures::future::join_all;

use crate::{config::CONFIG, CONTENT_ASSET_PK};

pub async fn delete_assets_data(ids: Vec<String>) -> Result<()> {
    let items = ids
        .into_iter()
        .map(|id| {
            WriteRequest::builder()
                .delete_request(
                    DeleteRequest::builder()
                        .key("PK", AttributeValue::S(CONTENT_ASSET_PK.to_owned()))
                        .key("SK", AttributeValue::S(id))
                        .build(),
                )
                .build()
        })
        .collect::<Vec<_>>()
        .chunks(DYNAMODB_MAX_BATCH_SIZE)
        .map(|chunk| chunk.into())
        .collect::<Vec<Vec<_>>>();

    join_all(items.into_iter().map(write_batch))
        .await
        .into_iter()
        .collect::<Result<Vec<_>>>()
        .map(|_| ())
}

async fn write_batch(items: Vec<WriteRequest>) -> Result<()> {
    let client = get_dynamodb_client().await;

    client
        .batch_write_item()
        .request_items(&CONFIG.dynamodb_table, items)
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            common_domain::error::Error::unknown(format!("Failed to write modifications: {e:?}"))
        })
}
