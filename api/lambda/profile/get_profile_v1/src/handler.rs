use common_api::lambda::{into_response::IntoResponse, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use profile_dto::{FromModel, ProfileDto};
use profile_infra::repository;
use use_case::profile::get_profile_by_id::{get_profile_by_id, GetProfileByIdRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .into_future()
        .and_then(|id| {
            get_profile_by_id(
                id,
                GetProfileByIdRepository {
                    get_profile_by_id: repository::get_profile_by_id,
                },
            )
        })
        .await
        .map(ProfileDto::from_model)
        .into_response(StatusCode::OK)
}
