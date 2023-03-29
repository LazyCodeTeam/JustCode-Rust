use crate::config::CONFIG;
use common_domain::error::Result;

pub async fn get_s3_object_url(key: impl Into<String>) -> Result<String> {
    Ok(format!("{}/{}", CONFIG.bucket_base_url, key.into()))
}
