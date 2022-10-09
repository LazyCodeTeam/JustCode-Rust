use std::fmt::Display;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use common_domain::error::{Error, ErrorType};

use super::error_dto::ErrorDto;

#[derive(PartialEq, Eq, Debug)]
pub struct ErrorResponseDto {
    pub status_code: StatusCode,
    pub error_dto: ErrorDto,
}

impl Display for ErrorResponseDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Error> for ErrorResponseDto {
    fn from(error: Error) -> Self {
        let status_code = match error.error_type {
            ErrorType::InvalidData => StatusCode::BAD_REQUEST,
            ErrorType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self {
            status_code,
            error_dto: ErrorDto::from(*error.details),
        }
    }
}

impl ResponseError for ErrorResponseDto {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(&self.error_dto)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_error() {
        let error = Error::unknown("");

        let result = ErrorResponseDto::from(error.clone());

        assert_eq!(result.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result.error_dto, ErrorDto::from(*error.details));
    }
}
