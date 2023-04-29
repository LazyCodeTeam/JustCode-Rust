mod dto;

use aws_lambda_events::cognito::CognitoEventUserPoolsPostConfirmation;
use common_api::lambda::register_internal_handler::register_internal_handler;
use dto::create_profile_dto::CreateProfileDto;
use lambda_runtime::{Error, LambdaEvent};
use profile_domain::model::create_profile_params::CreateProfileParams;
use use_case::profile::create_profile::{create_profile, CreateProfileRepository};

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_internal_handler(handle_request).await
}

pub async fn handle_request(
    event: LambdaEvent<CognitoEventUserPoolsPostConfirmation>,
) -> Result<CognitoEventUserPoolsPostConfirmation, Error> {
    let params =
        CreateProfileDto::try_from(event.payload.clone()).map(CreateProfileParams::from)?;

    create_profile(
        params,
        CreateProfileRepository {
            get_profile_by_id: profile_infra::repository::get_profile_by_id,
            save_profile: profile_infra::repository::save_profile,
        },
    )
    .await?;

    Ok(event.payload)
}
