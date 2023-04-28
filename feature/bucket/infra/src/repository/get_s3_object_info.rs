use crate::config::CONFIG;
use bucket_domain::model::bucket_object_head::BucketObjectHead;
use common_domain::error::{Result, ResultLogExt};
use common_infra::s3_client::get_s3_client;
use snafu::{OptionExt, ResultExt};

pub async fn get_s3_object_info(key: impl Into<String>) -> Result<BucketObjectHead> {
    let key: String = key.into();
    let client = get_s3_client().await;
    let head = client
        .head_object()
        .bucket(&CONFIG.s3_bucket)
        .key(&key)
        .send()
        .await
        .map_err(Box::new)
        .unwrap();

    let mut parts = key.split('/').collect::<Vec<_>>();
    let name = parts
        .pop()
        .whatever_context("Failed to get object name")
        .with_error_log()?;
    let prefix = parts.join("/") + "/";

    Ok(BucketObjectHead {
        name: name.to_owned(),
        prefix,
        mime: head
            .content_type()
            .whatever_context("Failed to get object content type")
            .with_error_log()?
            .to_owned(),
        size: head
            .content_length()
            .try_into()
            .whatever_context("content_length is invalid on object")
            .with_error_log()?,
    })
}
