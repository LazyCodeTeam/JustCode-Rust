use common_api::lambda::{into_response::IntoResponse, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::profile::set_push_data::{set_push_data, SetPushDataRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .get_user_id()
        .into_future()
        .and_then(|id| {
            set_push_data(
                (id, None),
                SetPushDataRepository {
                    update_push_data: profile_infra::repository::update_push_data,
                    remove_push_data: profile_infra::repository::remove_push_data,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::OK)
}
