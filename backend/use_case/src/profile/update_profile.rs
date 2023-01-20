use common_domain::error::{ErrorOutput, ErrorType, Result};
use profile_domain::{
    model::update_profile_params::UpdateProfileParams,
    port::{GetProfileById, UpdateProfile},
};

pub struct UpdateProfileRepository<T, Y> {
    pub get_profile_by_id: T,
    pub update_profile: Y,
}

pub async fn update_profile<T, Y>(
    (id, params): (String, UpdateProfileParams),
    repo: UpdateProfileRepository<T, Y>,
) -> Result<()>
where
    for<'a> T: GetProfileById<'a>,
    Y: UpdateProfile,
{
    let profile = (repo.get_profile_by_id)(&id).await?;
    match profile {
        Some(profile) => (repo.update_profile)(profile.update(params)).await,
        None => Err(not_found_error(&id)),
    }
}

fn not_found_error(id: &str) -> common_domain::error::Error {
    common_domain::error::Error {
        debug_message: format!("profile {} not found", id),
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

        let _get_profile_id_lock = profile_domain::port::get_profile_by_id_lock().await;
        let ctx = profile_domain::port::mock_get_profile_by_id::call_context();
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(None));

        let _update_profile_lock = profile_domain::port::update_profile_lock().await;
        let ctx = profile_domain::port::mock_update_profile::call_context();
        ctx.expect().never();

        let repo = UpdateProfileRepository {
            get_profile_by_id: profile_domain::port::mock_get_profile_by_id::call,
            update_profile: profile_domain::port::mock_update_profile::call,
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
            updated_at: now,
            created_at: now,
            ..Default::default()
        };

        let _get_profile_id_lock = profile_domain::port::get_profile_by_id_lock().await;
        let ctx = profile_domain::port::mock_get_profile_by_id::call_context();
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(move |_| Ok(Some(profile.clone())));

        let _update_profile_lock = profile_domain::port::update_profile_lock().await;
        let ctx = profile_domain::port::mock_update_profile::call_context();
        ctx.expect()
            .withf(move |profile| {
                profile.id == "id"
                    && profile.name == "name"
                    && profile.email == "email"
                    && profile.first_name == Some("first_name".to_owned())
                    && profile.last_name == Some("last_name".to_owned())
                    && profile.updated_at > now
                    && profile.created_at == now
            })
            .times(1)
            .returning(|_| Ok(()));

        let repo = UpdateProfileRepository {
            get_profile_by_id: profile_domain::port::mock_get_profile_by_id::call,
            update_profile: profile_domain::port::mock_update_profile::call,
        };

        let result = update_profile((id, update_params), repo).await;

        assert!(result.is_ok());
    }
}
