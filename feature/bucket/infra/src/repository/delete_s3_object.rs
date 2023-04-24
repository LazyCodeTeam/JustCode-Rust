use crate::config::CONFIG;
use common_domain::error::{Error, Result};
use common_infra::s3_client::get_s3_client;

pub async fn delete_s3_object(key: impl Into<String>) -> Result<()> {
    get_s3_client()
        .await
        .delete_object()
        .bucket(&CONFIG.s3_bucket)
        .key(key)
        .send()
        .await
        .map(|_| ())
        .map_err(|e| Error::unknown(format!("Failed to delete object: {e:?}")))
}
