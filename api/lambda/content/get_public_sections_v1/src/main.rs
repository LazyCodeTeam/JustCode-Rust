use common_api::lambda::{
    into_response::IntoResponse, lambda_error::LambdaError, lambda_request_ext::LambdaRequestExt,
    register_handler::register_handler,
};
use content_dto::{MapInto, SectionDto};
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use use_case::content::get_public_sections::{get_public_sections, GetPublicSectionsRepo};

const TECHNOLOGY_ID_PARAM_NAME: &str = "technology_id";

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let technology_id = event.required_path_parameter(TECHNOLOGY_ID_PARAM_NAME)?;

    get_public_sections(
        technology_id.to_owned(),
        GetPublicSectionsRepo {
            get_sections: content_infra::repository::get_ordered_technology_sections,
        },
    )
    .await
    .map::<Vec<SectionDto>, _>(MapInto::map_into)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
