use common_domain::{
    define_repo,
    error::{ErrorOutput, ErrorType, Result},
};
use profile_domain::model::profile::Profile;
use profile_domain::model::update_profile_params::UpdateProfileParams;

define_repo! {
    pub struct UpdateProfileRepository<T, Y> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as T,
        pub update_profile: Fn(params: Profile) -> Result<()> as Y,
    }
}

pub async fn update_profile<T, Y>(
    (id, params): (String, UpdateProfileParams),
    repo: UpdateProfileRepository<T, Y>,
) -> Result<()>
where
    T: GetProfileByIdType,
    Y: UpdateProfileType,
{
    let profile = (repo.get_profile_by_id)(&id).await?;
    match profile {
        Some(profile) => (repo.update_profile)(profile.update(params)).await,
        None => Err(not_found_error(&id)),
    }
}

fn not_found_error(id: &str) -> common_domain::error::Error {
    common_domain::error::Error {
        debug_message: format!("profile {id} not found"),
        error_type: ErrorType::NotFound,
        output: Box::new(ErrorOutput {
            message: "profile not found".to_string(),
            code: "profile_not_found".to_string(),
            ..Default::default()
        }),
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
        assert_eq!(result.unwrap_err(), not_found_error("id"));
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
