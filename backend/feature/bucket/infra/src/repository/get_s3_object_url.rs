use crate::config::CONFIG;
use common_domain::error::{Error, Result};

pub async fn get_s3_object_url(key: impl Into<String>) -> Result<String> {
    aws_config::load_from_env()
        .await
        .region()
        .ok_or_else(|| Error::unknown("Region not found".to_owned()))
        .map(|region| {
            format!(
                "https://{}.s3.{}.amazonaws.com/{}",
                &CONFIG.s3_bucket,
                region,
                key.into()
            )
        })
}
