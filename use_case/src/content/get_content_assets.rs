use common_domain::{
    define_repo,
    error::{Error, Result},
};
use content_domain::model::content_asset::ContentAsset;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct GetContentAssetsRepository<A> {
        pub get_assets: Fn() -> Result<Vec<ContentAsset>> as A,
    }
}

#[derive(Debug, Snafu)]
pub enum GetContentAssetsError {
    Infra { source: Error },
}

pub async fn get_content_assets<A>(
    repo: GetContentAssetsRepository<A>,
) -> std::result::Result<Vec<ContentAsset>, GetContentAssetsError>
where
    A: GetAssetsType,
{
    (repo.get_assets)().await.context(InfraSnafu)
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[tokio::test]
    async fn test_get_content_assets() {
        let assets = vec![
            ContentAsset {
                id: "id1".to_string(),
                content_type: "content_type1".to_string(),
                url: "url1".to_string(),
                created_at: Utc::now(),
            },
            ContentAsset {
                id: "id2".to_string(),
                content_type: "content_type2".to_string(),
                url: "url2".to_string(),
                created_at: Utc::now(),
            },
        ];
        let (ctx, _get_assets_lock) = mock_get_assets::ctx().await;
        let output = assets.clone();
        ctx.expect().return_once(move || Ok(output)).once();

        let repo = GetContentAssetsRepository {
            get_assets: mock_get_assets::call,
        };

        let result = get_content_assets(repo).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), assets);
    }
}
