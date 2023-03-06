use crate::dto::error_dto::ErrorDto;
use common_domain::error::{Error, ErrorType};
use http::StatusCode;
use lambda_http::{Body, Response};

#[derive(PartialEq, Eq, Debug)]
pub struct LambdaErrorDto {
    pub status_code: StatusCode,
    pub error_dto: ErrorDto,
}

impl From<Error> for LambdaErrorDto {
    fn from(error: Error) -> Self {
        Self {
            status_code: map_error_type(error.error_type),
            error_dto: (*error.output).into(),
        }
    }
}

fn map_error_type(error_type: ErrorType) -> StatusCode {
    match error_type {
        ErrorType::InvalidInput => StatusCode::BAD_REQUEST,
        ErrorType::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        ErrorType::Conflict => StatusCode::CONFLICT,
        ErrorType::NotModified => StatusCode::NOT_MODIFIED,
        ErrorType::NotFound => StatusCode::NOT_FOUND,
        ErrorType::Forbidden => StatusCode::FORBIDDEN,
        ErrorType::Unauthorized => StatusCode::UNAUTHORIZED,
    }
}

impl TryFrom<LambdaErrorDto> for Response<Body> {
    type Error = lambda_http::Error;

    fn try_from(error: LambdaErrorDto) -> Result<Self, Self::Error> {
        let result = Response::builder()
            .status(error.status_code)
            .header(
                http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.to_string(),
            )
            .body(
                serde_json::to_string(&error.error_dto)
                    .map_err(Box::new)?
                    .into(),
            )
            .map_err(Box::new)?;

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::dto::error_dto::ErrorDto;
    use common_domain::error::Error;
    use http::StatusCode;

    #[test]
    fn from_error() {
        let error = Error::unknown("".to_owned());

        let result = LambdaErrorDto::from(error.clone());

        assert_eq!(result.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(result.error_dto, ErrorDto::from(*error.output));
    }

    #[test]
    fn try_from() {
        let error = Error::unknown("".to_owned());

        let result = LambdaErrorDto::from(error);

        let response = Response::builder()
            .status(result.status_code)
            .header(
                http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.to_string(),
            )
            .body(
                serde_json::to_string(&result.error_dto)
                    .map_err(Box::new)
                    .unwrap()
                    .into(),
            )
            .map_err(Box::new)
            .unwrap();

        let result = Response::try_from(result).unwrap();

        assert_eq!(result.body(), response.body());
        assert_eq!(result.status(), response.status());
        assert_eq!(result.headers(), response.headers());
    }

    #[test]
    fn map_status_code() {
        assert_eq!(
            map_error_type(ErrorType::InvalidInput),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            map_error_type(ErrorType::Unknown),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(map_error_type(ErrorType::Conflict), StatusCode::CONFLICT);
        assert_eq!(map_error_type(ErrorType::NotFound), StatusCode::NOT_FOUND);
        assert_eq!(map_error_type(ErrorType::Forbidden), StatusCode::FORBIDDEN);
        assert_eq!(
            map_error_type(ErrorType::Unauthorized),
            StatusCode::UNAUTHORIZED
        );
    }
}
