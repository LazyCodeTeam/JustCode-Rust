use common_domain::error::Result;
use profile_domain::{
    model::create_profile_params::CreateProfileParams,
    port::{GetProfileById, SaveProfile},
};

pub struct CreateProfileRepository<T, Y> {
    pub get_profile_by_id: T,
    pub save_profile: Y,
}

pub async fn create_profile<T, Y>(
    params: CreateProfileParams,
    repo: CreateProfileRepository<T, Y>,
) -> Result<()>
where
    for<'a> T: GetProfileById<'a>,
    Y: SaveProfile,
{
    let profile = (repo.get_profile_by_id)(&params.id).await?;
    if profile.is_some() {
        return Ok(());
    }

    (repo.save_profile)(params).await
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
        let _get_profile_id_lock = profile_domain::port::get_profile_by_id_lock().await;
        let ctx = profile_domain::port::mock_get_profile_by_id::call_context();
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(None));

        let _save_profile_lock = profile_domain::port::save_profile_lock().await;
        let ctx = profile_domain::port::mock_save_profile::call_context();
        ctx.expect()
            .with(predicate::eq(input.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let repo = CreateProfileRepository {
            get_profile_by_id: profile_domain::port::mock_get_profile_by_id::call,
            save_profile: profile_domain::port::mock_save_profile::call,
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
        let _get_profile_id_lock = profile_domain::port::get_profile_by_id_lock().await;
        let ctx = profile_domain::port::mock_get_profile_by_id::call_context();
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| {
                Ok(Some(profile_domain::model::profile::Profile {
                    id: "id".to_string(),
                    name: "other_name".to_string(),
                    email: "other_email".to_string(),
                    avatar_url: None,
                }))
            });

        let _save_profile_lock = profile_domain::port::save_profile_lock().await;
        let ctx = profile_domain::port::mock_save_profile::call_context();
        ctx.expect().never();

        let repo = CreateProfileRepository {
            get_profile_by_id: profile_domain::port::mock_get_profile_by_id::call,
            save_profile: profile_domain::port::mock_save_profile::call,
        };

        let result = create_profile(input, repo).await;

        assert!(result.is_ok());
    }
}
