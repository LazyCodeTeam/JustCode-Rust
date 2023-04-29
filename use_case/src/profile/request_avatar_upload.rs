use bucket_domain::model::presigned_url::PresignedUrl;
use common_domain::{
    define_repo,
    error::{Error, Result, ResultLogExt},
};
use profile_domain::model::profile::Profile;
use snafu::{ResultExt, Snafu};

const UPLOAD_AVATAR_VALID_FOR: u64 = 60; // sec

define_repo! {
    pub struct RequestAvatarUploadRepository<A, B> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as A,
        pub get_avatar_upload_url: Fn(key: String, valid_fro: u64) -> Result<PresignedUrl> as B,
    }
}

#[derive(Debug, Snafu)]
pub enum RequestAvatarUploadError {
    NotFound,
    Infra { source: Error },
}

pub async fn request_avatar_upload<A, B>(
    profile_id: String,
    repo: RequestAvatarUploadRepository<A, B>,
) -> std::result::Result<PresignedUrl, RequestAvatarUploadError>
where
    A: GetProfileByIdType,
    B: GetAvatarUploadUrlType,
{
    let profile = (repo.get_profile_by_id)(&profile_id)
        .await
        .context(InfraSnafu)?;

    if profile.is_none() {
        return Err(RequestAvatarUploadError::NotFound).with_debug_log();
    }

    (repo.get_avatar_upload_url)(profile_id, UPLOAD_AVATAR_VALID_FOR)
        .await
        .context(InfraSnafu)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use common_domain::tokio;
    use profile_domain::model::profile::Profile;

    use super::*;

    #[tokio::test]
    async fn should_return_error_when_profile_not_created() {
        let id = "id";
        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(None));

        let (ctx, _get_avatar_upload_url_lock) = mock_get_avatar_upload_url::ctx().await;
        ctx.expect().times(0);

        let result = request_avatar_upload(
            id.to_owned(),
            RequestAvatarUploadRepository {
                get_profile_by_id: mock_get_profile_by_id::call,
                get_avatar_upload_url: mock_get_avatar_upload_url::call,
            },
        )
        .await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            RequestAvatarUploadError::NotFound
        ));
    }

    #[tokio::test]
    async fn should_return_presigned_url() {
        let date_time = chrono::Utc::now();
        let id = "id";
        let (ctx, _get_profile_id_lock) = mock_get_profile_by_id::ctx().await;
        ctx.expect()
            .withf(move |id| id == "id")
            .times(1)
            .returning(|_| Ok(Some(Profile::default())));

        let (ctx, _get_avatar_upload_url_lock) = mock_get_avatar_upload_url::ctx().await;
        ctx.expect()
            .withf(move |id, _| id == "id")
            .times(1)
            .returning(move |_, _| {
                Ok(PresignedUrl {
                    url: "url".to_owned(),
                    presigned_url: "url".to_owned(),
                    valid_until: date_time,
                    headers: HashMap::new(),
                })
            });

        let result = request_avatar_upload(
            id.to_owned(),
            RequestAvatarUploadRepository {
                get_profile_by_id: mock_get_profile_by_id::call,
                get_avatar_upload_url: mock_get_avatar_upload_url::call,
            },
        )
        .await;

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            PresignedUrl {
                url: "url".to_owned(),
                presigned_url: "url".to_owned(),
                valid_until: date_time,
                headers: HashMap::new(),
            }
        );
    }
}
