use crate::error::{
    DeserializationSnafu, MissingPathParameterSnafu, MissingQueryParameterSnafu,
    MissingUserIdSnafu, MissingUserNameSnafu,
};
use common_domain::error::ResultLogExt;
use lambda_http::{request::RequestContext, Request, RequestExt};
use serde::Deserialize;
use snafu::{OptionExt, ResultExt};

use crate::MapInto;

use super::lambda_error::LambdaError;

const USER_ID_KEY: &str = "sub";
const USERNAME_KEY: &str = "username";

pub trait LambdaRequestExt {
    fn user_id(&self) -> Result<String, LambdaError>;

    fn username(&self) -> Result<String, LambdaError>;

    fn path_parameter(&self, name: &str) -> Option<String>;

    fn deserialized_body<T>(&self) -> Result<T, LambdaError>
    where
        for<'a> T: Deserialize<'a>;

    fn required_path_parameter(&self, name: &str) -> Result<String, LambdaError> {
        self.path_parameter(name)
            .context(MissingPathParameterSnafu { name })
            .with_debug_log()
            .map_err(MapInto::map_into)
    }

    fn query_parameter(&self, name: &str) -> Option<String>;

    fn required_query_parameter(&self, name: &str) -> Result<String, LambdaError> {
        self.query_parameter(name)
            .context(MissingQueryParameterSnafu { name })
            .with_debug_log()
            .map_err(MapInto::map_into)
    }
}

impl LambdaRequestExt for Request {
    fn user_id(&self) -> Result<String, LambdaError> {
        match self.request_context() {
            RequestContext::ApiGatewayV2(context) => context
                .authorizer
                .and_then(|claims| claims.jwt)
                .and_then(|claims| {
                    claims
                        .claims
                        .get(USER_ID_KEY)
                        .map(|sub| sub.replace('-', ""))
                })
                .context(MissingUserIdSnafu)
                .with_warn_log()
                .map_err(MapInto::map_into),
        }
    }

    fn username(&self) -> Result<String, LambdaError> {
        match self.request_context() {
            RequestContext::ApiGatewayV2(context) => context
                .authorizer
                .and_then(|claims| claims.jwt)
                .and_then(|claims| {
                    claims
                        .claims
                        .get(USERNAME_KEY)
                        .map(|username| username.to_owned())
                })
                .context(MissingUserNameSnafu)
                .with_warn_log()
                .map_err(MapInto::map_into),
        }
    }

    fn path_parameter(&self, name: &str) -> Option<String> {
        self.path_parameters()
            .first(name)
            .map(|value| value.to_owned())
    }

    fn query_parameter(&self, name: &str) -> Option<String> {
        self.query_string_parameters()
            .first(name)
            .map(|value| value.to_owned())
    }

    fn deserialized_body<T>(&self) -> Result<T, LambdaError>
    where
        for<'a> T: Deserialize<'a>,
    {
        serde_json::from_slice::<T>(self.body())
            .context(DeserializationSnafu)
            .with_debug_log()
            .map_err(MapInto::map_into)
    }
}
