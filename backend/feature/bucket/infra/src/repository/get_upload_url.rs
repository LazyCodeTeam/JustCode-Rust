use crate::config::CONFIG;
use std::time::Duration;

use aws_sdk_s3::{model::ObjectCannedAcl, presigning::config::PresigningConfig};
use bucket_domain::model::presigned_url::PresignedUrl;
use chrono::Utc;
use common_domain::error::{Error, Result};
use common_infra::s3_client::get_s3_client;

pub async fn get_upload_url(key: &str, valid_for: u64) -> Result<PresignedUrl> {
    let client = get_s3_client().await;

    client
        .put_object()
        .bucket(&CONFIG.s3_bucket)
        .key(key)
        .acl(ObjectCannedAcl::PublicRead)
        .presigned(presigned_config(valid_for)?)
        .await
        .map_err(|err| Error::unknown(format!("Failed to presign avatar image  {key}: {err:?}")))
        .map(|presigned| PresignedUrl {
            url: presigned.uri().to_string(),
            valid_until: Utc::now() + url_chrono_duration(valid_for),
            headers: presigned
                .headers()
                .into_iter()
                .map(|(key, value)| {
                    (
                        key.to_string(),
                        value.to_str().unwrap_or_default().to_owned(),
                    )
                })
                .collect(),
        })
}

fn url_chrono_duration(valid_for: u64) -> chrono::Duration {
    chrono::Duration::seconds(valid_for.try_into().unwrap_or(i64::MAX))
}

fn url_std_duration(valid_for: u64) -> Duration {
    Duration::from_secs(valid_for)
}

fn presigned_config(valid_for: u64) -> Result<PresigningConfig> {
    PresigningConfig::expires_in(url_std_duration(valid_for)).map_err(|_| presigning_config_error())
}

fn presigning_config_error() -> Error {
    Error::unknown("Failed to generate presigning config".to_owned())
}
