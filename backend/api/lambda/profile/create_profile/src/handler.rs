use aws_lambda_events::cognito::CognitoEventUserPoolsPostConfirmation;
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_runtime::{Error, LambdaEvent};
use profile_domain::model::create_profile_params::CreateProfileParams;
use use_case::profile::create_profile::{create_profile, CreateProfileRepository};

use crate::dto::create_profile_dto::CreateProfileDto;

pub async fn handle_request(
    event: LambdaEvent<CognitoEventUserPoolsPostConfirmation>,
) -> Result<CognitoEventUserPoolsPostConfirmation, Error> {
    CreateProfileDto::try_from(event.payload.clone())
        .map(CreateProfileParams::from)
        .into_future()
        .and_then(|params| {
            create_profile(
                params,
                CreateProfileRepository {
                    get_profile_by_id: profile_infra::repository::get_profile_by_id,
                    save_profile: profile_infra::repository::save_profile,
                },
            )
        })
        .await
        .map_err(Box::new)?;

    Ok(event.payload)
}
