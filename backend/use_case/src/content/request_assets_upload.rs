use bucket_domain::model::presigned_url::PresignedUrl;
use common_domain::{define_repo, error::Result};
use content_domain::consts::CONTENT_ASSETS_PREFIX;

const UPLOAD_URL_VALID_FOR: u64 = 60; // sec

define_repo! {
    pub struct RequestAssetsUploadRepository<A> {
        pub get_upload_url: Fn(key: String, valid_fro: u64) -> Result<PresignedUrl> as A,
    }
}

pub async fn request_assets_upload<A>(
    count: u16,
    repo: RequestAssetsUploadRepository<A>,
) -> Result<Vec<PresignedUrl>>
where
    A: GetUploadUrlType,
{
    let keys: Vec<String> = (0..count)
        .map(|_| format!("{}{}", CONTENT_ASSETS_PREFIX, uuid::Uuid::new_v4().simple()))
        .collect();

    futures::future::join_all(
        keys.into_iter()
            .map(|key| (repo.get_upload_url)(key, UPLOAD_URL_VALID_FOR)),
    )
    .await
    .into_iter()
    .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn request_assets_upload_success() {
        let (ctx, _get_upload_url_lock) = mock_get_upload_url::ctx().await;
        ctx.expect()
            .withf(|key, _| key.starts_with(CONTENT_ASSETS_PREFIX))
            .times(3)
            .returning(|_, _| Ok(PresignedUrl::default()));

        let repo = RequestAssetsUploadRepository {
            get_upload_url: mock_get_upload_url::call,
        };

        let urls = request_assets_upload(3, repo).await.unwrap();

        assert_eq!(urls.len(), 3);
    }
}
