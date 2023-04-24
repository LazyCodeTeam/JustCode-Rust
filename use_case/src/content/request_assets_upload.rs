use bucket_domain::model::presigned_url::PresignedUrl;
use common_domain::{define_repo, error::Result};

const UPLOAD_URL_VALID_FOR: u64 = 120; // sec - 2 min

define_repo! {
    pub struct RequestAssetsUploadRepository<A> {
        pub get_upload_url: Fn(valid_fro: u64) -> Result<PresignedUrl> as A,
    }
}

pub async fn request_assets_upload<A>(
    count: u16,
    repo: RequestAssetsUploadRepository<A>,
) -> Result<Vec<PresignedUrl>>
where
    A: GetUploadUrlType,
{
    futures::future::join_all((0..count).map(|_| (repo.get_upload_url)(UPLOAD_URL_VALID_FOR)))
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
            .times(3)
            .returning(|_| Ok(PresignedUrl::default()));

        let repo = RequestAssetsUploadRepository {
            get_upload_url: mock_get_upload_url::call,
        };

        let urls = request_assets_upload(3, repo).await.unwrap();

        assert_eq!(urls.len(), 3);
    }
}
