use crate::{config::CONFIG, dto::profile_dto::ProfileDto, MapFrom};
use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use profile_domain::model::create_profile_params::CreateProfileParams;
use serde_dynamo::to_item;
use snafu::ResultExt;

pub async fn save_profile(params: CreateProfileParams) -> Result<()> {
    let dto = ProfileDto::map_from(params);

    save_serialized_profile(dto).await
}

pub(crate) async fn save_serialized_profile(dto: ProfileDto) -> Result<()> {
    let client = get_dynamodb_client().await;

    let item = to_item(&dto)
        .whatever_context("Failed to serialize profile")
        .with_error_log()?;

    client
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to save profile")
        .with_error_log()
}
