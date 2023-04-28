use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use profile_domain::model::profile::Profile;
use profile_domain::model::update_profile_params::UpdateProfileParams;
use snafu::{ResultExt, Snafu};

define_repo! {
    pub struct UpdateProfileRepository<T, Y> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as T,
        pub update_profile: Fn(params: Profile) -> Result<()> as Y,
    }
}

#[derive(Debug, Snafu)]
pub enum UpdateProfileError {
    NotFound,
    Infra { source: Error },
}

pub async fn update_profile<T, Y>(
    (id, params): (String, UpdateProfileParams),
    repo: UpdateProfileRepository<T, Y>,
) -> std::result::Result<(), UpdateProfileError>
where
    T: GetProfileByIdType,
    Y: UpdateProfileType,
{
    let profile = (repo.get_profile_by_id)(&id).await.context(InfraSnafu)?;
    match profile {
        Some(profile) => (repo.update_profile)(profile.update(params))
            .await
            .context(InfraSnafu),
        None => Err(UpdateProfileError::NotFound).with_debug_log(),
    }
}

#[cfg(test)]
mod test {
    use profile_domain::model::profile::Profile;

    use super::*;

    #[tokio::test]
    async fn profile_not_found() {
        let id = "id".to_string();
        let update_params = UpdateProfileParams {
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
        };

        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(None));

        let (ctx, _update_profile_lock) = mock_update_profile::ctx().await;
        ctx.expect().never();

        let repo = UpdateProfileRepository {
            get_profile_by_id: mock_get_profile_by_id::call,
            update_profile: mock_update_profile::call,
        };

        let result = update_profile((id, update_params), repo).await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), UpdateProfileError::NotFound));
    }

    #[tokio::test]
    async fn successful_update() {
        let now = chrono::Utc::now();
        let id = "id".to_string();
        let update_params = UpdateProfileParams {
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
        };
        let profile = Profile {
            id: "id".to_string(),
            name: "name".to_string(),
            email: "email".to_string(),
            created_at: now,
            ..Default::default()
        };

        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(move |_| Ok(Some(profile.clone())));

        let (ctx, _update_profile_lock) = mock_update_profile::ctx().await;
        ctx.expect()
            .withf(move |profile| {
                profile.id == "id"
                    && profile.name == "name"
                    && profile.email == "email"
                    && profile.first_name == Some("first_name".to_owned())
                    && profile.last_name == Some("last_name".to_owned())
                    && profile.created_at == now
            })
            .times(1)
            .returning(|_| Ok(()));

        let repo = UpdateProfileRepository {
            get_profile_by_id: mock_get_profile_by_id::call,
            update_profile: mock_update_profile::call,
        };

        let result = update_profile((id, update_params), repo).await;

        assert!(result.is_ok());
    }
}
