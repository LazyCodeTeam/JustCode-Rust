use std::collections::HashMap;

use common_domain::{
    define_repo,
    error::{Error, ErrorOutput, Result},
};
use futures::{future::join_all, FutureExt};

define_repo! {
    pub struct DeleteContentAssetsRepository<A, B> {
        pub delete_asset_object: Fn(id: String) -> Result<()> as A,
        pub delete_assets_data: Fn(ids: Vec<String>) -> Result<()> as B,
    }
}

pub async fn delete_content_assets<A, B>(
    ids: Vec<String>,
    repo: DeleteContentAssetsRepository<A, B>,
) -> Result<()>
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
        .filter_map(|(id, r)| r.to_owned().map_err(|e| (id.to_owned(), e)).err())
        .collect::<Vec<_>>();

    let ids = results
        .into_iter()
        .filter_map(|(id, r)| r.map(|_| id).ok())
        .collect::<Vec<_>>();

    (repo.delete_assets_data)(ids).await?;

    if failed.is_empty() {
        Ok(())
    } else {
        Err(failed_to_delete_objects_error(failed))
    }
}

fn failed_to_delete_objects_error(failed: Vec<(String, Error)>) -> Error {
    Error {
        debug_message: "Failed to delete some objects".to_owned(),
        output: Box::new(ErrorOutput {
            message: "Some objects were not deleted".to_owned(),
            code: "not_all_objects_deleted".to_owned(),
            args: failed
                .into_iter()
                .map(|(id, error)| (id, error.to_string()))
                .collect::<HashMap<_, _>>(),
        }),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
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
        let error = Error::unknown("error".to_owned());
        let (ctx, _delete_asset_object_lock) = mock_delete_asset_object::ctx().await;
        ctx.expect()
            .with(mockall::predicate::eq("id_1".to_owned()))
            .times(1)
            .returning(|_| Ok(()));
        let out = error.clone();
        ctx.expect()
            .with(mockall::predicate::eq("id_2".to_owned()))
            .times(1)
            .return_once(move |_| Err(out));

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
        assert_eq!(
            result.unwrap_err(),
            failed_to_delete_objects_error(vec![("id_2".to_owned(), error)])
        );
    }
}
