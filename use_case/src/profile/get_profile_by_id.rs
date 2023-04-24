use std::collections::HashMap;

use common_domain::{
    define_repo,
    error::{Error, ErrorOutput, ErrorType, Result},
};
use profile_domain::model::profile::Profile;

define_repo! {
    pub struct GetProfileByIdRepository<T> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as T,
    }
}

pub async fn get_profile_by_id<T>(id: String, repo: GetProfileByIdRepository<T>) -> Result<Profile>
where
    T: GetProfileByIdType,
{
    (repo.get_profile_by_id)(&id).await.and_then(|result| {
        result.ok_or_else(|| Error {
            debug_message: format!("Profile with id {id} not found"),
            error_type: ErrorType::NotFound,
            output: Box::new(ErrorOutput {
                message: "Profile not found".to_string(),
                code: "profile_not_found".to_string(),
                args: HashMap::new(),
            }),
        })
    })
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
        assert_eq!(result.unwrap_err().error_type, ErrorType::NotFound);
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
