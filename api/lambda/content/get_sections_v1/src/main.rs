use common_api::lambda::into_response::IntoResponse;
use common_api::lambda::lambda_error::LambdaError;
use common_api::lambda::lambda_request_ext::LambdaRequestExt;
use common_api::lambda::register_handler::register_handler;
use content_dto::{MapFrom, MapInto, PersonalizedSectionDto};
use lambda_http::Error;
use lambda_http::{http::StatusCode, Body, Request, Response};
use use_case::content::get_sections::{get_sections, GetSectionsRepo};

const TECHNOLOGY_ID_PARAM_NAME: &str = "technology_id";

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_handler(handle_request).await
}

pub async fn handle_request(event: Request) -> Result<Response<Body>, LambdaError> {
    let technology_id = event.required_path_parameter(TECHNOLOGY_ID_PARAM_NAME)?;
    let user_id = event.user_id()?;
    get_sections(
        technology_id,
        user_id,
        GetSectionsRepo {
            get_sections: content_infra::repository::get_ordered_technology_sections,
            get_valid_historical_answers: content_infra::repository::get_first_valid_answers,
        },
    )
    .await
    .map(Vec::<PersonalizedSectionDto>::map_from)
    .map_err(MapInto::map_into)?
    .into_response(StatusCode::OK)
}
