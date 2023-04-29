use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use futures::{future::join_all, FutureExt};
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct DeleteContentAssetsRepository<A, B> {
        pub delete_asset_object: Fn(id: String) -> Result<()> as A,
        pub delete_assets_data: Fn(ids: Vec<String>) -> Result<()> as B,
    }
}

#[derive(Debug, Snafu)]
pub enum DeleteContentAssetsError {
    #[snafu(display("Failed to delete some assets: {failed:?}"))]
    SomeAssetsNotDeleted {
        failed: Vec<String>,
    },
    Infra {
        source: Error,
    },
}

pub async fn delete_content_assets<A, B>(
    ids: Vec<String>,
    repo: DeleteContentAssetsRepository<A, B>,
) -> std::result::Result<(), DeleteContentAssetsError>
where
    A: DeleteAssetObjectType,
    B: DeleteAssetsDataType,
{
    let results = join_all(
        ids.into_iter()
            .map(|id| (repo.delete_asset_object)(id.to_owned()).map(move |r| (id, r))),
    )
    .await;

    let failed = results
        .iter()
        .filter(|(_, r)| r.is_err())
        .map(|(id, _)| id.to_owned())
        .collect::<Vec<_>>();

    let ids = results
        .into_iter()
        .filter_map(|(id, r)| r.map(|_| id).ok())
        .collect::<Vec<_>>();

    (repo.delete_assets_data)(ids).await.context(InfraSnafu)?;

    if failed.is_empty() {
        Ok(())
    } else {
        Err(DeleteContentAssetsError::SomeAssetsNotDeleted { failed })
    }
    .with_debug_log()
}

#[cfg(test)]
mod tests {
    use snafu::whatever;

    use super::*;

    #[tokio::test]
    async fn successful_deletion() {
        let (ctx, _delete_asset_object_lock) = mock_delete_asset_object::ctx().await;
        ctx.expect()
            .with(mockall::predicate::eq("id_1".to_owned()))
            .times(1)
            .returning(|_| Ok(()));
        ctx.expect()
            .with(mockall::predicate::eq("id_2".to_owned()))
            .times(1)
            .returning(|_| Ok(()));

        let (ctx, _delete_assets_data_lock) = mock_delete_assets_data::ctx().await;
        ctx.expect()
            .with(mockall::predicate::eq(vec![
                "id_1".to_owned(),
                "id_2".to_owned(),
            ]))
            .times(1)
            .returning(|_| Ok(()));

        let repo = DeleteContentAssetsRepository {
            delete_asset_object: mock_delete_asset_object::call,
            delete_assets_data: mock_delete_assets_data::call,
        };

        let result = delete_content_assets(vec!["id_1".to_owned(), "id_2".to_owned()], repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn not_all_objects_deleted() {
        let (ctx, _delete_asset_object_lock) = mock_delete_asset_object::ctx().await;
        ctx.expect()
            .with(mockall::predicate::eq("id_1".to_owned()))
            .times(1)
            .returning(|_| Ok(()));
        ctx.expect()
            .with(mockall::predicate::eq("id_2".to_owned()))
            .times(1)
            .return_once(move |_| whatever!(""));

        let (ctx, _delete_assets_data_lock) = mock_delete_assets_data::ctx().await;
        ctx.expect()
            .with(mockall::predicate::eq(vec!["id_1".to_owned()]))
            .times(1)
            .returning(|_| Ok(()));

        let repo = DeleteContentAssetsRepository {
            delete_asset_object: mock_delete_asset_object::call,
            delete_assets_data: mock_delete_assets_data::call,
        };

        let result = delete_content_assets(vec!["id_1".to_owned(), "id_2".to_owned()], repo).await;

        assert!(result.is_err());
        assert!(match result.unwrap_err() {
            DeleteContentAssetsError::SomeAssetsNotDeleted { failed } => {
                failed == vec!["id_2".to_owned()]
            }
            _ => false,
        });
    }
}
