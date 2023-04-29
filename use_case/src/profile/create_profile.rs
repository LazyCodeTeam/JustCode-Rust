use common_domain::{
    define_repo,
    error::{Error, Result},
};
use profile_domain::model::{create_profile_params::CreateProfileParams, profile::Profile};
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct CreateProfileRepository<T, Y> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as T,
        pub save_profile: Fn(params: CreateProfileParams) -> Result<()> as Y,
    }
}

#[derive(Debug, Snafu)]
pub enum CreateProfileError {
    Infra { source: Error },
}

pub async fn create_profile<T, Y>(
    params: CreateProfileParams,
    repo: CreateProfileRepository<T, Y>,
) -> std::result::Result<(), CreateProfileError>
where
    T: GetProfileByIdType,
    Y: SaveProfileType,
{
    let profile = (repo.get_profile_by_id)(&params.id)
        .await
        .context(InfraSnafu)?;
    if profile.is_some() {
        return Ok(());
    }

    (repo.save_profile)(params).await.context(InfraSnafu)
}

#[cfg(test)]
mod test {
    use common_domain::tokio;
    use mockall::predicate;

    use super::*;

    #[tokio::test]
    async fn success() {
        let input = CreateProfileParams {
            id: "id".to_owned(),
            ..Default::default()
        };
        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(None));

        let (ctx, _save_profile_lock) = mock_save_profile::ctx().await;
        ctx.expect()
            .with(predicate::eq(input.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let repo = CreateProfileRepository {
            get_profile_by_id: mock_get_profile_by_id::call,
            save_profile: mock_save_profile::call,
        };

        let result = create_profile(input, repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn already_exist() {
        let input = CreateProfileParams {
            id: "id".to_string(),
            name: "name".to_string(),
            email: "email".to_string(),
        };
        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| {
                Ok(Some(profile_domain::model::profile::Profile {
                    id: "id".to_string(),
                    name: "other_name".to_string(),
                    email: "other_email".to_string(),
                    ..Default::default()
                }))
            });

        let (ctx, _save_profile_lock) = mock_save_profile::ctx().await;
        ctx.expect().never();

        let repo = CreateProfileRepository {
            get_profile_by_id: mock_get_profile_by_id::call,
            save_profile: mock_save_profile::call,
        };

        let result = create_profile(input, repo).await;

        assert!(result.is_ok());
    }
}
