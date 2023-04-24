use http::StatusCode;
use lambda_http::{Body, Error, Response};

use crate::dto::lambda_error_dto::LambdaErrorDto;

pub trait IntoResponse {
    fn into_response(self, status_code: StatusCode) -> Result<Response<Body>, Error>;
}

pub trait IntoEmptyRespone {
    fn into_empty_response(self, status_code: StatusCode) -> Result<Response<Body>, Error>;
}
impl<T> IntoResponse for Result<T, common_domain::error::Error>
where
    T: serde::Serialize,
{
    fn into_response(self, status_code: StatusCode) -> Result<Response<Body>, Error> {
        match self {
            Ok(value) => Ok(Response::builder()
                .status(status_code)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&value).map_err(Box::new)?.into())
                .map_err(Box::new)?),
            Err(err) => {
                err.log();

                LambdaErrorDto::from(err).try_into()
            }
        }
    }
}

impl<T> IntoEmptyRespone for Result<T, common_domain::error::Error> {
    fn into_empty_response(self, status_code: StatusCode) -> Result<Response<Body>, Error> {
        match self {
            Ok(_) => Ok(Response::builder()
                .status(status_code)
                .body(Body::Empty)
                .map_err(Box::new)?),
            Err(err) => {
                err.log();

                LambdaErrorDto::from(err).try_into()
            }
        }
    }
}
