use common_api::lambda::{
    from_request::FromRequest, into_response::IntoEmptyRespone, user_context::UserContext,
};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_domain::model::update_profile_params::UpdateProfileParams;
use profile_dto::{FromDto, UpdateProfileDto};
use profile_infra::repository;
use use_case::profile::update_profile::{update_profile, UpdateProfileRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    UpdateProfileDto::from_request(&event)
        .and_then(|dto| Ok((event.get_user_id()?, UpdateProfileParams::from_dto(dto))))
        .into_future()
        .and_then(|params| {
            update_profile(
                params,
                UpdateProfileRepository {
                    get_profile_by_id: repository::get_profile_by_id,
                    update_profile: repository::update_profile,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::OK)
}
