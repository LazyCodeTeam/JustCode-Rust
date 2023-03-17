use bucket_domain::model::presigned_url::PresignedUrl;
use common_domain::{
    define_repo,
    error::{Error, ErrorOutput, ErrorType, Result},
};
use profile_domain::consts::AVATAR_IMAGE_PREFIX;
use profile_domain::model::profile::Profile;

const UPLOAD_AVATAR_VALID_FOR: u64 = 60; // sec

define_repo! {
    pub struct RequestAvatarUploadRepository<A, B> {
        pub get_profile_by_id: Fn<'a>(id: &'a str) -> Result<Option<Profile>> as A,
        pub get_avatar_upload_url: Fn(key: String, valid_fro: u64) -> Result<PresignedUrl> as B,
    }
}

pub async fn request_avatar_upload<A, B>(
    profile_id: String,
    repo: RequestAvatarUploadRepository<A, B>,
) -> Result<PresignedUrl>
where
    A: GetProfileByIdType,
    B: GetAvatarUploadUrlType,
{
    let profile = (repo.get_profile_by_id)(&profile_id).await?;

    if profile.is_none() {
        return Err(profile_not_created(&profile_id));
    }

    (repo.get_avatar_upload_url)(
        format!("{AVATAR_IMAGE_PREFIX}{profile_id}"),
        UPLOAD_AVATAR_VALID_FOR,
    )
    .await
}

fn profile_not_created(profile_id: &str) -> Error {
    Error {
        debug_message: format!("Profile {profile_id} not created, cannot upload avatar"),
        error_type: ErrorType::Conflict,
        output: Box::new(ErrorOutput {
            message: "Profile not created, cannot upload avatar".to_owned(),
            code: "profile_not_created".to_owned(),
            ..Default::default()
        }),
    }
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
        assert_eq!(result.as_ref().unwrap_err().error_type, ErrorType::Conflict);
        assert_eq!(result.unwrap_err(), profile_not_created(id));
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
            .withf(move |id, _| id == "profile/avatar/id")
            .times(1)
            .returning(move |_, _| {
                Ok(PresignedUrl {
                    url: "url".to_owned(),
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
                valid_until: date_time,
                headers: HashMap::new(),
            }
        );
    }
}
