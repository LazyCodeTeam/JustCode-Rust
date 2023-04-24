use crate::config::CONFIG;
use bucket_domain::model::bucket_object_head::BucketObjectHead;
use common_domain::error::{Error, Result};
use common_infra::s3_client::get_s3_client;

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
        .ok_or_else(|| Error::unknown(format!("key is invalid: {key}")))?;
    let prefix = parts.join("/") + "/";

    Ok(BucketObjectHead {
        name: name.to_owned(),
        prefix,
        mime: head
            .content_type()
            .ok_or_else(|| Error::unknown(format!("content_type is empty on object {key}")))?
            .to_owned(),
        size: head.content_length().try_into().map_err(|e| {
            Error::unknown(format!("content_length is invalid on object {key}: {e:?}",))
        })?,
    })
}
