use common_api::lambda::into_response::IntoResponse;
use content_dto::{IntoDto, TechnologyDto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_public_technologies::{
    get_public_technologies, GetPublicTechnologiesRepo,
};

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    get_public_technologies(GetPublicTechnologiesRepo {
        get_technologies: content_infra::repository::get_ordered_technologies,
    })
    .await
    .map(|technologies| {
        technologies
            .into_iter()
            .map(IntoDto::into_dto)
            .collect::<Vec<TechnologyDto>>()
    })
    .into_response(StatusCode::OK)
}
