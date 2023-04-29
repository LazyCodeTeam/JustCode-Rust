use std::collections::HashMap;

use gen::models::ErrorDto;
use http::StatusCode;
use snafu::Snafu;

use crate::{lambda::lambda_error::LambdaError, MapFrom};

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum CommonApiError {
    #[snafu(display("Failed to build response"))]
    BuildResponse { source: http::Error },
    #[snafu(display("Failed to serialize response body: {}", source))]
    Serialization { source: serde_json::Error },
    #[snafu(display("Failed to deserialize request body: {}", source))]
    Deserialization { source: serde_json::Error },
    #[snafu(display("Failed to read user id"))]
    MissingUserId,
    #[snafu(display("Failed to read user name"))]
    MissingUserName,
    #[snafu(display("Failed to read query parameter: {}", name))]
    MissingQueryParameter { name: String },
    #[snafu(display("Failed to read path parameter: {}", name))]
    MissingPathParameter { name: String },
}

impl MapFrom<CommonApiError> for LambdaError {
    fn map_from(error: CommonApiError) -> Self {
        match error {
            CommonApiError::BuildResponse { .. } => LambdaError::internal_server_error(),
            CommonApiError::Serialization { .. } => LambdaError::internal_server_error(),
            CommonApiError::Deserialization { .. } => LambdaError {
                code: StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    message: "Invalid json".to_string(),
                    code: "invalid_json".to_string(),
                    ..Default::default()
                },
            },
            // Should never happen, gateway should always provide user id
            CommonApiError::MissingUserId { .. } => LambdaError::internal_server_error(),
            // Should never happen, gateway should always provide user name
            CommonApiError::MissingUserName { .. } => LambdaError::internal_server_error(),
            CommonApiError::MissingQueryParameter { name } => LambdaError {
                code: StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    message: format!("Missing query parameter: {}", name),
                    code: "missing_query_parameter".to_string(),
                    args: HashMap::from([("name".to_string(), name)]),
                },
            },
            CommonApiError::MissingPathParameter { name } => LambdaError {
                code: StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    message: format!("Missing path parameter: {}", name),
                    code: "missing_path_parameter".to_string(),
                    args: HashMap::from([("name".to_string(), name)]),
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn map_from_build_response_error() {
        let error = CommonApiError::BuildResponse {
            source: http::Response::builder().status(0).body(()).unwrap_err(),
        };

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(lambda_error, LambdaError::internal_server_error(),);
    }

    #[test]
    fn map_from_serialization_error() {
        let mut map = BTreeMap::new();
        map.insert(vec![32, 64], "x86");
        let error = CommonApiError::Serialization {
            source: serde_json::to_value(map).unwrap_err(),
        };

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(lambda_error, LambdaError::internal_server_error(),);
    }

    #[test]
    fn map_from_deserialization_error() {
        let mut map = BTreeMap::new();
        map.insert(vec![32, 64], "x86");
        let error = CommonApiError::Deserialization {
            source: serde_json::to_value(map).unwrap_err(),
        };

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(
            lambda_error,
            LambdaError {
                code: StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    message: "Invalid json".to_string(),
                    code: "invalid_json".to_string(),
                    ..Default::default()
                },
            },
        );
    }

    #[test]
    fn map_from_missing_user_id_error() {
        let error = CommonApiError::MissingUserId;

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(lambda_error, LambdaError::internal_server_error(),);
    }

    #[test]
    fn map_from_missing_user_name_error() {
        let error = CommonApiError::MissingUserName;

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(lambda_error, LambdaError::internal_server_error(),);
    }

    #[test]
    fn map_from_missing_query_parameter_error() {
        let error = CommonApiError::MissingQueryParameter {
            name: "test".to_string(),
        };

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(
            lambda_error,
            LambdaError {
                code: StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    message: "Missing query parameter: test".to_string(),
                    code: "missing_query_parameter".to_string(),
                    args: HashMap::from([("name".to_string(), "test".to_string())]),
                },
            },
        );
    }

    #[test]
    fn map_from_missing_path_parameter_error() {
        let error = CommonApiError::MissingPathParameter {
            name: "test".to_string(),
        };

        let lambda_error = LambdaError::map_from(error);

        assert_eq!(
            lambda_error,
            LambdaError {
                code: StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    message: "Missing path parameter: test".to_string(),
                    code: "missing_path_parameter".to_string(),
                    args: HashMap::from([("name".to_string(), "test".to_string())]),
                },
            },
        );
    }
}
