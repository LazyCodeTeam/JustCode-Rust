use crate::error::{BuildResponseSnafu, SerializationSnafu};
use common_domain::error::ResultLogExt;
use http::StatusCode;
use lambda_http::{Body, Response};
use snafu::ResultExt;

use crate::MapInto;

use super::lambda_error::LambdaError;

pub trait IntoResponse {
    fn into_response(self, status_code: StatusCode) -> Result<Response<Body>, LambdaError>;
}

pub trait IntoEmptyRespone {
    fn into_empty_response(self, status_code: StatusCode) -> Result<Response<Body>, LambdaError>;
}
impl<T> IntoResponse for T
where
    T: serde::Serialize,
{
    fn into_response(self, status_code: StatusCode) -> Result<Response<Body>, LambdaError> {
        Response::builder()
            .status(status_code)
            .header("Content-Type", "application/json")
            .body(
                serde_json::to_string(&self)
                    .context(SerializationSnafu)
                    .map_err(MapInto::map_into)?
                    .into(),
            )
            .context(BuildResponseSnafu)
            .with_error_log()
            .map_err(MapInto::map_into)
    }
}

impl<T> IntoEmptyRespone for T {
    fn into_empty_response(self, status_code: StatusCode) -> Result<Response<Body>, LambdaError> {
        Response::builder()
            .status(status_code)
            .body(Body::Empty)
            .context(BuildResponseSnafu)
            .with_error_log()
            .map_err(MapInto::map_into)
    }
}
