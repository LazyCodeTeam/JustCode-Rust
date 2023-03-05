use common_api::lambda::into_response::IntoResponse;
use common_domain::into_future::IntoFuture;
use content_dto::output::section_dto::SectionDto;
use futures::TryFutureExt;
use lambda_http::{http::StatusCode, Body, Error, Request, RequestExt, Response};
use use_case::content::get_public_sections::{get_public_sections, GetPublicSectionsRepo};

const TECHNOLOGY_ID_PARAM_NAME: &str = "technology_id";

pub async fn handle_request(event: Request) -> Result<Response<Body>, Error> {
    event
        .path_parameters()
        .first(TECHNOLOGY_ID_PARAM_NAME)
        .ok_or_else(|| common_domain::error::Error::unknown("Failed to get technology_id from url"))
        .into_future()
        .and_then(|technology_id| {
            get_public_sections(
                technology_id.to_owned(),
                GetPublicSectionsRepo {
                    get_sections: content_infra::repository::get_technology_sections,
                },
            )
        })
        .await
        .map(|sections| {
            sections
                .into_iter()
                .map(Into::into)
                .collect::<Vec<SectionDto>>()
        })
        .into_response(StatusCode::OK)
}
