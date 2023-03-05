use common_api::lambda::into_response::IntoResponse;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_technologies::{get_technologies, GetTechnologiesRepo};

use crate::dto::technology_dto::TechnologyDto;

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    get_technologies(GetTechnologiesRepo {
        get_all_technologies: content_infra::repository::get_all_technologies,
    })
    .await
    .map(|technologies| {
        technologies
            .into_iter()
            .map(Into::into)
            .collect::<Vec<TechnologyDto>>()
    })
    .into_response(StatusCode::OK)
}