use common_domain::{
    define_repo,
    error::{Error, Result},
};
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct DeleteProfileRepository<A, B> {
        pub delete_current_user: FnOnce() -> Result<()> as A,
        pub delete_current_profile: FnOnce() -> Result<()> as B,
    }
}

#[derive(Debug, Snafu)]
pub enum DeleteProfileError {
    Infra { source: Error },
}

pub async fn delete_profile<A, B>(
    repo: DeleteProfileRepository<A, B>,
) -> std::result::Result<(), DeleteProfileError>
where
    A: DeleteCurrentUserType,
    B: DeleteCurrentProfileType,
{
    (repo.delete_current_user)().await.context(InfraSnafu)?;

    (repo.delete_current_profile)().await.context(InfraSnafu)
}

#[cfg(test)]
mod tests {
    use snafu::whatever;

    use super::*;

    #[tokio::test]
    async fn delete_profile_and_user() {
        let (ctx, _delete_profile_lock) = mock_delete_current_profile::ctx().await;
        ctx.expect().returning(|| Ok(())).once();

        let (ctx, _delete_user_lock) = mock_delete_current_user::ctx().await;
        ctx.expect().returning(|| Ok(())).once();

        let repo = DeleteProfileRepository {
            delete_current_user: mock_delete_current_user::call,
            delete_current_profile: mock_delete_current_profile::call,
        };

        let result = delete_profile(repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_skip_profile_delete() {
        let (ctx, _delete_profile_lock) = mock_delete_current_profile::ctx().await;
        ctx.expect().never();

        let (ctx, _delete_user_lock) = mock_delete_current_user::ctx().await;
        ctx.expect().returning(|| whatever!("")).once();

        let repo = DeleteProfileRepository {
            delete_current_user: mock_delete_current_user::call,
            delete_current_profile: mock_delete_current_profile::call,
        };

        let result = delete_profile(repo).await;

        assert!(result.is_err());
    }
}
