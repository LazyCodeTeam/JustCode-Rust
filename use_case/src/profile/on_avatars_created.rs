use bucket_domain::model::bucket_object_head::BucketObjectHead;
use common_domain::{
    define_repo,
    error::{Error, Result},
};
use futures::future::join_all;
use snafu::{ResultExt, Snafu};

const ALLOWED_CONTENT_TYPES: [&str; 2] = ["image/png", "image/jpeg"];
const MAX_SIZE: u64 = 1024 * 1024 * 3; // 3 MB

define_repo! {
    pub struct OnAvatarsCreatedRepository<A, B, C, D> {
        pub get_bucket_object_info: Fn(key: String) -> Result<BucketObjectHead> as A,
        pub delete_bucket_object: Fn(key: String) -> Result<()> as B,
        pub update_profile_avatar: Fn(id: String, url: Option<String>) -> Result<()> as C,
        pub get_bucket_object_url: Fn(key: String) -> Result<String> as D,
    }
}

#[derive(Debug, Snafu)]
pub enum OnAvatarsCreatedError {
    Infra { source: Error },
}

pub async fn on_avatars_created<A, B, C, D>(
    users_ids: Vec<String>,
    repo: OnAvatarsCreatedRepository<A, B, C, D>,
) -> std::result::Result<(), OnAvatarsCreatedError>
where
    A: GetBucketObjectInfoType,
    B: DeleteBucketObjectType,
    C: UpdateProfileAvatarType,
    D: GetBucketObjectUrlType,
{
    join_all(
        users_ids
            .into_iter()
            .map(|id| on_single_avatar_created(id, &repo)),
    )
    .await
    .into_iter()
    .collect::<Result<Vec<_>>>()
    .map(|_| ())
    .context(InfraSnafu)
}

async fn on_single_avatar_created<A, B, C, D>(
    id: String,
    repo: &OnAvatarsCreatedRepository<A, B, C, D>,
) -> Result<()>
where
    A: GetBucketObjectInfoType,
    B: DeleteBucketObjectType,
    C: UpdateProfileAvatarType,
    D: GetBucketObjectUrlType,
{
    let bucket_object_head = (repo.get_bucket_object_info)(id.clone()).await?;

    if !ALLOWED_CONTENT_TYPES.contains(&bucket_object_head.mime.as_str())
        || bucket_object_head.size > MAX_SIZE
    {
        log::info!("Deleting invalid avatar {}: {:?}", id, bucket_object_head);
        (repo.update_profile_avatar)(id.clone(), None).await?;

        return (repo.delete_bucket_object)(id).await;
    }
    let avatar_url = (repo.get_bucket_object_url)(id.clone()).await?;

    (repo.update_profile_avatar)(id, Some(avatar_url)).await
}

#[cfg(test)]
mod test {
    use bucket_domain::model::bucket_object_head::BucketObjectHead;
    use common_domain::tokio;

    use super::*;

    #[tokio::test]
    async fn avatar_mime_not_allowed() {
        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/gif".to_string(),
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(move |id, avatar_url| id == "1" && avatar_url.is_none())
            .times(1)
            .return_once(move |_, _| Ok(()));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .returning(|_| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect().times(0);

        let repo = OnAvatarsCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result = on_single_avatar_created("1".to_owned(), &repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn avatar_too_big() {
        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            size: 1024 * 1024 * 4,
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(move |id, avatar_url| id == "1" && avatar_url.is_none())
            .times(1)
            .return_once(move |_, _| Ok(()));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .returning(|_| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect().times(0);

        let repo = OnAvatarsCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result = on_single_avatar_created("1".to_owned(), &repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn success() {
        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            ..Default::default()
        };
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .return_once(move |_| Ok(object_head));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect().times(0);

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(|id, url| id == "1" && url == &Some("https://example.com".to_owned()))
            .times(1)
            .returning(|_, _| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .return_once(move |_| Ok("https://example.com".to_string()));

        let repo = OnAvatarsCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result = on_single_avatar_created("1".to_owned(), &repo).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn update_multiple_success() {
        let (ctx, _get_bucket_object_info_lock) = mock_get_bucket_object_info::ctx().await;
        let object_head = BucketObjectHead {
            mime: "image/png".to_string(),
            ..Default::default()
        };
        let head_clone = object_head.clone();
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .return_once(move |_| Ok(head_clone));
        let head_clone = object_head.clone();
        ctx.expect()
            .withf(move |id| id == "2")
            .times(1)
            .return_once(move |_| Ok(head_clone));
        let head_clone = object_head.clone();
        ctx.expect()
            .withf(move |id| id == "3")
            .times(1)
            .return_once(move |_| Ok(head_clone));

        let (ctx, _delete_bucket_object_lock) = mock_delete_bucket_object::ctx().await;
        ctx.expect().times(0);

        let (ctx, _update_profile_avatar_lock) = mock_update_profile_avatar::ctx().await;
        ctx.expect()
            .withf(|id, url| id == "1" && url == &Some("https://example.com/1".to_owned()))
            .times(1)
            .returning(|_, _| Ok(()));
        ctx.expect()
            .withf(|id, url| id == "2" && url == &Some("https://example.com/2".to_owned()))
            .times(1)
            .returning(|_, _| Ok(()));
        ctx.expect()
            .withf(|id, url| id == "3" && url == &Some("https://example.com/3".to_owned()))
            .times(1)
            .returning(|_, _| Ok(()));

        let (ctx, _get_bucket_object_url_lock) = mock_get_bucket_object_url::ctx().await;
        ctx.expect()
            .withf(move |id| id == "1")
            .times(1)
            .return_once(move |_| Ok("https://example.com/1".to_string()));
        ctx.expect()
            .withf(move |id| id == "2")
            .times(1)
            .return_once(move |_| Ok("https://example.com/2".to_string()));
        ctx.expect()
            .withf(move |id| id == "3")
            .times(1)
            .return_once(move |_| Ok("https://example.com/3".to_string()));

        let repo = OnAvatarsCreatedRepository {
            get_bucket_object_info: mock_get_bucket_object_info::call,
            delete_bucket_object: mock_delete_bucket_object::call,
            update_profile_avatar: mock_update_profile_avatar::call,
            get_bucket_object_url: mock_get_bucket_object_url::call,
        };

        let result =
            on_avatars_created(vec!["1".to_owned(), "2".to_owned(), "3".to_owned()], repo).await;

        assert!(result.is_ok());
    }
}
