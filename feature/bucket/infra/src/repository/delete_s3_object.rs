use crate::config::CONFIG;
use common_domain::error::{Result, ResultLogExt};
use common_infra::s3_client::get_s3_client;
use snafu::ResultExt;

pub async fn delete_s3_object(key: impl Into<String>) -> Result<()> {
    get_s3_client()
        .await
        .delete_object()
        .bucket(&CONFIG.s3_bucket)
        .key(key)
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to delete object")
        .with_error_log()
}
