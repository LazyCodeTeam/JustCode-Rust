use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use common_domain::error::{Error, ErrorType};

use super::error_dto::ErrorDto;

#[derive(PartialEq, Eq, Debug)]
pub struct ErrorResponseDto {
    pub status_code: StatusCode,
    pub error_dto: ErrorDto,
}

impl Display for ErrorResponseDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl IntoResponse for ErrorResponseDto {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.error_dto)).into_response()
    }
}

impl From<Error> for ErrorResponseDto {
    fn from(error: Error) -> Self {
        Self {
            status_code: map_error_type(error.error_type),
            error_dto: ErrorDto::from(*error.output),
        }
    }
}

fn map_error_type(error_type: ErrorType) -> StatusCode {
    match error_type {
        ErrorType::InvalidInput => StatusCode::BAD_REQUEST,
        ErrorType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        ErrorType::Conflict => StatusCode::CONFLICT,
        ErrorType::NotFound => StatusCode::NOT_FOUND,
        ErrorType::Forbidden => StatusCode::FORBIDDEN,
        ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_error() {
        let error = Error::unknown("".to_owned());

        let result = ErrorResponseDto::from(error.clone());

        assert_eq!(result.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result.error_dto, ErrorDto::from(*error.output));
    }

    #[test]
    fn from_error_type() {
        let error = Error::unknown("".to_owned());

        let result = ErrorResponseDto::from(error.clone());

        assert_eq!(result.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result.error_dto, ErrorDto::from(*error.output));
    }

    #[test]
    fn map_error_type_invalid_input() {
        let error_type = ErrorType::InvalidInput;

        let result = map_error_type(error_type);

        assert_eq!(result, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn map_error_type_unknown() {
        let error_type = ErrorType::Unknown;

        let result = map_error_type(error_type);

        assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn map_error_type_conflict() {
        let error_type = ErrorType::Conflict;

        let result = map_error_type(error_type);

        assert_eq!(result, StatusCode::CONFLICT);
    }

    #[test]
    fn map_error_type_not_found() {
        let error_type = ErrorType::NotFound;

        let result = map_error_type(error_type);

        assert_eq!(result, StatusCode::NOT_FOUND);
    }

    #[test]
    fn map_error_type_forbidden() {
        let error_type = ErrorType::Forbidden;

        let result = map_error_type(error_type);

        assert_eq!(result, StatusCode::FORBIDDEN);
    }

    #[test]
    fn map_error_type_unauthorized() {
        let error_type = ErrorType::Unauthorized;

        let result = map_error_type(error_type);

        assert_eq!(result, StatusCode::UNAUTHORIZED);
    }
}
