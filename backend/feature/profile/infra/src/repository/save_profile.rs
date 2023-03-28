use crate::{config::CONFIG, dto::profile_dto::ProfileDto, FromModel};
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use profile_domain::model::create_profile_params::CreateProfileParams;
use serde_dynamo::to_item;

pub async fn save_profile(params: CreateProfileParams) -> Result<()> {
    let dto = ProfileDto::from_model(params);

    save_serialized_profile(dto).await
}

pub(crate) async fn save_serialized_profile(dto: ProfileDto) -> Result<()> {
    let client = get_dynamodb_client().await;

    let item = to_item(&dto)
        .map_err(|e| Error::unknown(format!("Failed to serialize profile ({e:?}): {dto:?}")))?;

    client
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("Failed to save profile ({e:?}): {dto:?}")))
}
