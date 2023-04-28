use gen::models::ErrorDto;
use http::{Response, StatusCode};
use lambda_http::{aws_lambda_events::serde_json::json, Body};
use snafu::Snafu;

use crate::MapFrom;

#[derive(PartialEq, Debug, Snafu)]
pub struct LambdaError {
    pub code: StatusCode,
    pub dto: ErrorDto,
}

impl LambdaError {
    pub fn new(code: StatusCode, dto: ErrorDto) -> Self {
        Self { code, dto }
    }

    pub fn not_modified() -> Self {
        Self {
            code: StatusCode::NOT_MODIFIED,
            dto: ErrorDto {
                message: "Not Modified".to_string(),
                code: "not_modified".to_string(),
                ..Default::default()
            },
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            dto: ErrorDto {
                message: "Internal Server Error".to_string(),
                code: "internal_server_error".to_string(),
                ..Default::default()
            },
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: StatusCode::NOT_FOUND,
            dto: ErrorDto {
                message: "Not Found".to_string(),
                code: "not_found".to_string(),
                ..Default::default()
            },
        }
    }
}

impl MapFrom<LambdaError> for Response<Body> {
    fn map_from(error: LambdaError) -> Self {
        let Ok(body) = serde_json::to_string(&error.dto) else {
            return default_error_response();
        };

        let Ok(response) = Response::builder()
            .status(error.code)
            .body(Body::from(body)) else {
            return default_error_response();
        };

        response
    }
}

fn default_error_response() -> Response<Body> {
    let mut response = Response::new(Body::Text(
        json!({
            "message": "Internal Server Error",
            "code": "internal_server_error",
            "args": {}
        })
        .to_string(),
    ));
    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    response
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let error = LambdaError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            ErrorDto {
                message: "Internal Server Error".to_string(),
                code: "internal_server_error".to_string(),
                ..Default::default()
            },
        );

        assert_eq!(
            error,
            LambdaError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                dto: ErrorDto {
                    message: "Internal Server Error".to_string(),
                    code: "internal_server_error".to_string(),
                    ..Default::default()
                },
            }
        );
    }

    #[test]
    fn not_modified() {
        let error = LambdaError::not_modified();

        assert_eq!(
            error,
            LambdaError {
                code: StatusCode::NOT_MODIFIED,
                dto: ErrorDto {
                    message: "Not Modified".to_string(),
                    code: "not_modified".to_string(),
                    ..Default::default()
                },
            }
        );
    }

    #[test]
    fn internal_server_error() {
        let error = LambdaError::internal_server_error();

        assert_eq!(
            error,
            LambdaError {
                code: StatusCode::INTERNAL_SERVER_ERROR,
                dto: ErrorDto {
                    message: "Internal Server Error".to_string(),
                    code: "internal_server_error".to_string(),
                    ..Default::default()
                },
            }
        );
    }

    #[test]
    fn not_found() {
        let error = LambdaError::not_found();

        assert_eq!(
            error,
            LambdaError {
                code: StatusCode::NOT_FOUND,
                dto: ErrorDto {
                    message: "Not Found".to_string(),
                    code: "not_found".to_string(),
                    ..Default::default()
                },
            }
        );
    }

    #[test]
    fn map_from() {
        let error = LambdaError::internal_server_error();

        let response = Response::map_from(error);

        assert!(matches!(response.body(), Body::Text(_)));
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[test]
    fn test_default_error_response() {
        let response = default_error_response();

        assert!(matches!(response.body(), Body::Text(_)));
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
