use common_api::lambda::into_response::IntoEmptyRespone;
use common_api::lambda::{from_request::FromRequest, user_context::UserContext};
use common_domain::into_future::IntoFuture;
use futures::TryFutureExt;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Error, Request, Response};
use profile_domain::model::push_data::PushData;
use profile_dto::{FromDto, PushDataDto};
use use_case::profile::set_push_data::{set_push_data, SetPushDataRepository};

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    PushDataDto::from_request(&event)
        .and_then(|dto| Ok((event.get_user_id()?, PushData::from_dto(dto))))
        .into_future()
        .and_then(|(id, data)| {
            set_push_data(
                (id, Some(data)),
                SetPushDataRepository {
                    update_push_data: profile_infra::repository::update_push_data,
                    remove_push_data: profile_infra::repository::remove_push_data,
                },
            )
        })
        .await
        .into_empty_response(StatusCode::OK)
}
