use aws_lambda_events::s3;
use bucket_infra::consts::CONTENT_ASSETS_PREFIX;
use common_api::lambda::register_internal_handler::register_internal_handler;
use lambda_runtime::{Error, LambdaEvent};
use use_case::content::on_assets_uploaded::{on_assets_uploaded, OnAssetsUploadedRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_internal_handler(handle_event).await
}

pub async fn handle_event(event: LambdaEvent<s3::S3Event>) -> Result<(), Error> {
    let ids: Vec<String> = event
        .payload
        .records
        .into_iter()
        .filter_map(|record| record.s3.object.key)
        .map(|key| key.replace(CONTENT_ASSETS_PREFIX, ""))
        .collect();

    on_assets_uploaded(
        ids,
        OnAssetsUploadedRepository {
            get_bucket_object_info: |id| {
                bucket_infra::repository::get_s3_object_info(format!("{CONTENT_ASSETS_PREFIX}{id}"))
            },
            get_bucket_object_url: |id| {
                bucket_infra::repository::get_s3_object_url(format!("{CONTENT_ASSETS_PREFIX}{id}"))
            },
            save_content_asset: content_infra::repository::save_content_asset,
        },
    )
    .await?;

    Ok(())
}
