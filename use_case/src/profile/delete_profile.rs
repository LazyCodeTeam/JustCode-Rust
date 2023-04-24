use common_domain::{define_repo, error::Result};

define_repo! {
    pub struct DeleteProfileRepository<A, B> {
        pub delete_current_user: FnOnce() -> Result<()> as A,
        pub delete_current_profile: FnOnce() -> Result<()> as B,
    }
}

pub async fn delete_profile<A, B>(repo: DeleteProfileRepository<A, B>) -> Result<()>
where
    A: DeleteCurrentUserType,
    B: DeleteCurrentProfileType,
{
    (repo.delete_current_user)().await?;

    (repo.delete_current_profile)().await
}

#[cfg(test)]
mod tests {
    use common_domain::error::Error;

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
        ctx.expect().returning(|| Err(Error::default())).once();

        let repo = DeleteProfileRepository {
            delete_current_user: mock_delete_current_user::call,
            delete_current_profile: mock_delete_current_profile::call,
        };

        let result = delete_profile(repo).await;

        assert!(result.is_err());
    }
}
