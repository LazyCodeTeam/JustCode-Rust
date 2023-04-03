use common_api::lambda::{into_response::IntoEmptyRespone, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::profile::delete_profile::{delete_profile, DeleteProfileRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .and_then(|id| -> common_domain::error::Result<(String, String)> {
            Ok((id, event.get_username()?))
        })
        .into_future()
        .and_then(|(id, username)| {
            delete_profile(DeleteProfileRepository {
                delete_current_profile: || profile_infra::repository::delete_profile_by_id(id),
                delete_current_user: || user_infra::repository::delete_user_by_username(username),
            })
        })
        .await
        .into_empty_response(StatusCode::OK)
}
