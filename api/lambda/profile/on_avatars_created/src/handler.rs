use aws_lambda_events::s3;
use bucket_infra::consts::PROFILE_AVATARS_PREFIX;
use lambda_runtime::{Error, LambdaEvent};
use use_case::profile::on_avatars_created::{on_avatars_created, OnAvatarsCreatedRepository};

pub async fn handle_event(event: LambdaEvent<s3::S3Event>) -> Result<(), Error> {
    let ids: Vec<String> = event
        .payload
        .records
        .into_iter()
        .filter_map(|record| record.s3.object.key)
        .map(|key| key.replace(PROFILE_AVATARS_PREFIX, ""))
        .collect();

    on_avatars_created(
        ids,
        OnAvatarsCreatedRepository {
            get_bucket_object_info: |id| {
                bucket_infra::repository::get_s3_object_info(format!(
                    "{}{}",
                    PROFILE_AVATARS_PREFIX, id
                ))
            },
            delete_bucket_object: |id| {
                bucket_infra::repository::delete_s3_object(format!(
                    "{}{}",
                    PROFILE_AVATARS_PREFIX, id
                ))
            },
            get_bucket_object_url: |id| {
                bucket_infra::repository::get_s3_object_url(format!(
                    "{}{}",
                    PROFILE_AVATARS_PREFIX, id
                ))
            },
            update_profile_avatar: profile_infra::repository::update_profile_avatar,
        },
    )
    .await?;

    Ok(())
}
