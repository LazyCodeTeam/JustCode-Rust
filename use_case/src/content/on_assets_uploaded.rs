use bucket_domain::model::bucket_object_head::BucketObjectHead;
use common_domain::{define_repo, error::Result};
use content_domain::model::content_asset_creation_data::ContentAssetCreationData;
use futures::future::join_all;

define_repo! {
    pub struct OnAssetsUploadedRepository<A, B, C> {
        pub get_bucket_object_info: Fn(id: String) -> Result<BucketObjectHead> as A,
        pub get_bucket_object_url: Fn(id: String) -> Result<String> as B,
        pub save_content_asset: Fn(content_asset: ContentAssetCreationData) -> Result<()> as C,
    }
}

pub async fn on_assets_uploaded<A, B, C>(
    keys: Vec<String>,
    repo: OnAssetsUploadedRepository<A, B, C>,
) -> Result<()>
where
    A: GetBucketObjectInfoType,
    B: GetBucketObjectUrlType,
    C: SaveContentAssetType,
{
    join_all(
        keys.into_iter()
            .map(|id| on_single_asset_uploaded(id, &repo)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<()>>>()?;

    Ok(())
}

async fn on_single_asset_uploaded<A, B, C>(
    id: String,
    repo: &OnAssetsUploadedRepository<A, B, C>,
) -> Result<()>
where
    A: GetBucketObjectInfoType,
    B: GetBucketObjectUrlType,
    C: SaveContentAssetType,
{
    let info = (repo.get_bucket_object_info)(id.clone()).await?;
    let url = (repo.get_bucket_object_url)(id.clone()).await?;
    let content_asset = ContentAssetCreationData {
        id,
        content_type: info.mime,
        url,
    };

    (repo.save_content_asset)(content_asset).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn single_asset_uploaded() {
        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        ctx.expect()
            .withf(|id| id == "id")
            .returning(|_| {
                Ok(BucketObjectHead {
                    prefix: "prefix".to_string(),
                    name: "name".to_string(),
                    mime: "image/jpeg".to_string(),
                    size: 100,
                })
            })
            .once();

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect()
            .withf(|id| id == "id")
            .returning(|_| Ok("url".to_string()))
            .once();

        let (ctx, _save_content_asset_lock) = mock_save_content_asset::ctx().await;
        ctx.expect()
            .withf(|content_asset| {
                content_asset.id == "id"
                    && content_asset.content_type == "image/jpeg"
                    && content_asset.url == "url"
            })
            .returning(|_| Ok(()))
            .once();

        let repo = OnAssetsUploadedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
            save_content_asset: mock_save_content_asset::call,
        };

        let result = on_single_asset_uploaded("id".to_string(), &repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn assets_uploaded() {
        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        ctx.expect()
            .withf(|id| id == "id1")
            .returning(|_| {
                Ok(BucketObjectHead {
                    prefix: "prefix".to_string(),
                    name: "name".to_string(),
                    mime: "image/jpeg".to_string(),
                    size: 100,
                })
            })
            .once();
        ctx.expect()
            .withf(|id| id == "id2")
            .returning(|_| {
                Ok(BucketObjectHead {
                    prefix: "prefix".to_string(),
                    name: "name".to_string(),
                    mime: "image/jpeg".to_string(),
                    size: 100,
                })
            })
            .once();

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect()
            .withf(|id| id == "id1")
            .returning(|_| Ok("url1".to_string()))
            .once();
        ctx.expect()
            .withf(|id| id == "id2")
            .returning(|_| Ok("url2".to_string()))
            .once();

        let (ctx, _save_content_asset_lock) = mock_save_content_asset::ctx().await;
        ctx.expect()
            .withf(|content_asset| {
                content_asset.id == "id1"
                    && content_asset.content_type == "image/jpeg"
                    && content_asset.url == "url1"
            })
            .returning(|_| Ok(()))
            .once();
        ctx.expect()
            .withf(|content_asset| {
                content_asset.id == "id2"
                    && content_asset.content_type == "image/jpeg"
                    && content_asset.url == "url2"
            })
            .returning(|_| Ok(()))
            .once();

        let repo = OnAssetsUploadedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
            save_content_asset: mock_save_content_asset::call,
        };

        let result = on_assets_uploaded(vec!["id1".to_string(), "id2".to_string()], repo).await;

        assert!(result.is_ok());
    }
}
