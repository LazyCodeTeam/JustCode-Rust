use aws_lambda_events::s3;
use lambda_runtime::{Error, LambdaEvent};
use use_case::profile::on_avatar_created::{on_avatar_created, OnAvatarCreatedRepository};

pub async fn handle_event(event: LambdaEvent<s3::S3Event>) -> Result<(), Error> {
    let result = futures_util::future::join_all(
        event
            .payload
            .records
            .into_iter()
            .filter_map(|record| record.s3.object.key)
            .map(|key| {
                tokio::spawn(async move {
                    on_avatar_created(
                        key,
                        OnAvatarCreatedRepository {
                            get_bucket_object_info: bucket_infra::repository::get_s3_object_info,
                            delete_bucket_object: bucket_infra::repository::delete_s3_object,
                            update_profile_avatar: profile_infra::repository::update_profile_avatar,
                            get_bucket_object_url: bucket_infra::repository::get_s3_object_url,
                        },
                    )
                    .await
                })
            })
            .collect::<Vec<_>>(),
    )
    .await;

    let errors = result
        .into_iter()
        .filter_map(|r| r.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        let error_message = errors
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        log::error!("{}", error_message);

        return Err(Box::new(common_domain::error::Error::unknown(
            error_message,
        )));
    }

    Ok(())
}
