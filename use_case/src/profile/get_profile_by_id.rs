use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use profile_domain::model::profile::Profile;
use snafu::{OptionExt, ResultExt, Snafu};

define_repo! {
    pub struct GetProfileByIdRepository<T> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as T,
    }
}

#[derive(Debug, Snafu)]
pub enum GetProfileByIdError {
    NotFound,
    Infra { source: Error },
}

pub async fn get_profile_by_id<T>(
    id: String,
    repo: GetProfileByIdRepository<T>,
) -> std::result::Result<Profile, GetProfileByIdError>
where
    T: GetProfileByIdType,
{
    (repo.get_profile_by_id)(&id)
        .await
        .context(InfraSnafu)
        .and_then(|result| result.context(NotFoundSnafu).with_debug_log())
}

#[cfg(test)]
mod test {
    use common_domain::tokio;

    use super::*;

    #[tokio::test]
    async fn should_return_not_found() {
        let id = "id".to_owned();
        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .times(1)
            .withf(|arg| arg == "id")
            .returning(|_| Ok(None));

        let result = get_profile_by_id(
            id,
            GetProfileByIdRepository {
                get_profile_by_id: mock_get_profile_by_id::call,
            },
        )
        .await;

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            GetProfileByIdError::NotFound
        ));
    }

    #[tokio::test]
    async fn should_return_profile() {
        let id = "id".to_owned();
        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .times(1)
            .withf(|arg| arg == "id")
            .returning(|_| {
                Ok(Some(Profile {
                    id: "id".to_owned(),
                    name: "name".to_owned(),
                    email: "email".to_owned(),
                    avatar_url: Some("avatar_url".to_owned()),
                    first_name: Some("first_name".to_owned()),
                    last_name: Some("last_name".to_owned()),
                    ..Default::default()
                }))
            });

        let result = get_profile_by_id(
            id,
            GetProfileByIdRepository {
                get_profile_by_id: mock_get_profile_by_id::call,
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(result.as_ref().unwrap().name, "name");
        assert_eq!(result.as_ref().unwrap().email, "email");
        assert_eq!(
            result.as_ref().unwrap().avatar_url,
            Some("avatar_url".to_owned())
        );
        assert_eq!(
            result.as_ref().unwrap().first_name,
            Some("first_name".to_owned())
        );
        assert_eq!(
            result.as_ref().unwrap().avatar_url,
            Some("avatar_url".to_owned())
        );
    }
}
