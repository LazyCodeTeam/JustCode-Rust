use bucket_domain::model::bucket_object_head::BucketObjectHead;
use common_domain::{
    define_repo,
    error::{Error, Result},
};

const ALLOWED_CONTENT_TYPES: [&str; 2] = ["image/png", "image/jpeg"];
const MAX_SIZE: u64 = 1024 * 1024 * 3; // 3 MB

define_repo! {
    pub struct OnAvatarCreatedRepository<A, B, C, D> {
        pub get_bucket_object_info: Fn<'a>(key: &'a str) -> Result<BucketObjectHead> as A,
        pub delete_bucket_object: Fn<'a>(key: &'a str) -> Result<()> as B,
        pub update_profile_avatar: Fn<'a>(id: &'a str, url: Option<&'a str>) -> Result<()> as C,
        pub get_bucket_object_url: Fn<'a>(key: &'a str) -> Result<String> as D,
    }
}

pub async fn on_avatar_created<A, B, C, D>(
    key: String,
    repo: OnAvatarCreatedRepository<A, B, C, D>,
) -> Result<()>
where
    A: GetBucketObjectInfoType,
    B: DeleteBucketObjectType,
    C: UpdateProfileAvatarType,
    D: GetBucketObjectUrlType,
{
    let bucket_object_head = (repo.get_bucket_object_info)(&key).await?;
    let id = key.split('/').last().ok_or_else(|| {
        Error::unknown("Failed to split object key (should never happend)".to_owned())
    })?;

    if !ALLOWED_CONTENT_TYPES.contains(&bucket_object_head.mime.as_str())
        || bucket_object_head.size > MAX_SIZE
    {
        log::info!("Deleting invalid avatar {}: {:?}", key, bucket_object_head);
        (repo.update_profile_avatar)(id, None).await?;

        return (repo.delete_bucket_object)(&key).await;
    }
    let avatar_url = (repo.get_bucket_object_url)(&key).await?;

    (repo.update_profile_avatar)(id, Some(&avatar_url)).await
}

#[cfg(test)]
mod test {
    use bucket_domain::model::bucket_object_head::BucketObjectHead;
    use common_domain::tokio;

    use super::*;

    #[tokio::test]
    async fn avatar_mime_not_allowed() {
        let key = "profile/avatar/1".to_string();

        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/gif".to_string(),
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(move |id, avatar_url| id == "1" && avatar_url.is_none())
            .times(1)
            .return_once(move |_, _| Ok(()));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .returning(|_| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect().times(0);

        let repo = OnAvatarCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result = on_avatar_created(key, repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn avatar_too_big() {
        let key = "profile/avatar/1".to_string();

        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            size: 1024 * 1024 * 4,
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(move |id, avatar_url| id == "1" && avatar_url.is_none())
            .times(1)
            .return_once(move |_, _| Ok(()));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .returning(|_| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect().times(0);

        let repo = OnAvatarCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result = on_avatar_created(key, repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn success() {
        let key = "profile/avatar/1".to_string();

        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect().times(0);

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(|id, url| id == "1" && url.unwrap() == "https://example.com")
            .times(1)
            .returning(|_, _| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect()
            .withf(move |id| id == "profile/avatar/1")
            .times(1)
            .return_once(move |_| Ok("https://example.com".to_string()));

        let repo = OnAvatarCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result = on_avatar_created(key, repo).await;

        assert!(result.is_ok());
    }
}
