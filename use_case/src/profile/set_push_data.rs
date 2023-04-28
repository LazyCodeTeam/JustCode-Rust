use common_domain::{
    define_repo,
    error::{Error, Result},
};
use profile_domain::model::push_data::PushData;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct SetPushDataRepository<A, B> {
        pub update_push_data: Fn<'a>(id: &'a str, data: &'a PushData) -> Result<()> as A,
        pub remove_push_data: Fn<'a>(id: &'a str) -> Result<()> as B,
    }
}

#[derive(Debug, Snafu)]
pub enum SetPushDataError {
    Infra { source: Error },
}

pub async fn set_push_data<A, B>(
    (id, data): (String, Option<PushData>),
    repo: SetPushDataRepository<A, B>,
) -> std::result::Result<(), SetPushDataError>
where
    A: UpdatePushDataType,
    B: RemovePushDataType,
{
    match data {
        Some(data) => (repo.update_push_data)(&id, &data).await,
        None => (repo.remove_push_data)(&id).await,
    }
    .context(InfraSnafu)
}

#[cfg(test)]
mod test {
    use mockall::predicate::eq;
    use profile_domain::model::platform::Platform;

    use super::*;

    #[tokio::test]
    async fn updates_data() {
        let id = "123";
        let push_data = PushData {
            token: "token".to_owned(),
            platform: Platform::Android,
        };
        let (ctx, _update_push_data_lock) = mock_update_push_data::ctx().await;
        ctx.expect()
            .with(eq(id.to_owned()), eq(push_data.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let (ctx, _remove_push_data_lock) = mock_remove_push_data::ctx().await;
        ctx.expect().times(0);

        let repo = SetPushDataRepository {
            update_push_data: mock_update_push_data::call,
            remove_push_data: mock_remove_push_data::call,
        };

        let result = set_push_data((id.to_owned(), Some(push_data)), repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn removes_data() {
        let id = "123";
        let (ctx, _update_push_data_lock) = mock_update_push_data::ctx().await;
        ctx.expect().times(0);

        let (ctx, _remove_push_data_lock) = mock_remove_push_data::ctx().await;
        ctx.expect()
            .with(eq(id.to_owned()))
            .times(1)
            .returning(|_| Ok(()));

        let repo = SetPushDataRepository {
            update_push_data: mock_update_push_data::call,
            remove_push_data: mock_remove_push_data::call,
        };

        let result = set_push_data((id.to_owned(), None), repo).await;

        assert!(result.is_ok());
    }
}
