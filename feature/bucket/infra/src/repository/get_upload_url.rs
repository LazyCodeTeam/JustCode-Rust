use crate::config::CONFIG;
use std::time::Duration;

use aws_sdk_s3::presigning::PresigningConfig;
use bucket_domain::model::presigned_url::PresignedUrl;
use chrono::Utc;
use common_domain::error::{Result, ResultLogExt};
use common_infra::s3_client::get_s3_client;
use snafu::ResultExt;

use super::get_s3_object_url;

pub async fn get_upload_url<S, S2>(
    prefix: S,
    name: Option<S2>,
    valid_for: u64,
) -> Result<PresignedUrl>
where
    S: Into<String>,
    S2: Into<String>,
{
    let client = get_s3_client().await;
    let key = format!(
        "{}{}",
        prefix.into(),
        name.map::<String, _>(Into::into)
            .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string())
    );
    let url = get_s3_object_url(&key).await?;

    client
        .put_object()
        .bucket(&CONFIG.s3_bucket)
        .key(key)
        .presigned(presigned_config(valid_for)?)
        .await
        .whatever_context("Failed to presign avatar image")
        .with_error_log()
        .map(|presigned| PresignedUrl {
            presigned_url: presigned.uri().to_string(),
            url,
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
    PresigningConfig::expires_in(url_std_duration(valid_for))
        .whatever_context("Failed to generate presigning config")
        .with_error_log()
}
