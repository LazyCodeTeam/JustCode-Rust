use aws_sdk_dynamodb::model::WriteRequest;
use common_domain::error::Result;
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::modification::Modification;

use crate::{config::CONFIG, modification_dto::ModificationDto};

pub async fn write_modifications(modifications: Vec<Modification>) -> Result<()> {
    let items = modifications
        .into_iter()
        .map(ModificationDto::from)
        .map(WriteRequest::try_from)
        .collect::<Result<Vec<WriteRequest>>>()?;

    get_dynamodb_client()
        .await
        .batch_write_item()
        .request_items(&CONFIG.dynamodb_table, items)
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            common_domain::error::Error::unknown(format!("Failed to write modifications: {e:?}"))
        })?;
    Ok(())
}
