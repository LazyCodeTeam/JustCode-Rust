use aws_sdk_dynamodb::types::WriteRequest;
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use content_domain::model::modification::Modification;
use snafu::ResultExt;

use crate::{config::CONFIG, modification_dto::ModificationDto, MapFrom};

pub async fn write_modifications(modifications: Vec<Modification>) -> Result<()> {
    let items = modifications
        .into_iter()
        .map(ModificationDto::map_from)
        .map(WriteRequest::try_from)
        .collect::<Result<Vec<WriteRequest>>>()?;

    get_dynamodb_client()
        .await
        .batch_write_item()
        .request_items(&CONFIG.content_dynamodb_table, items)
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to write modifications")
        .with_error_log()?;
    Ok(())
}
