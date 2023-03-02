use common_api::lambda::into_response::IntoResponse;
use lambda_http::{http::StatusCode, Body, Error, Request, Response};
use load_tasks_v1::dto::technology_dto::TechnologyDto;

pub async fn handle_request(_event: Request) -> Result<Response<Body>, Error> {
    Ok(fake::vec![TechnologyDto; 100]).into_response::<Vec<TechnologyDto>>(StatusCode::OK)
}
