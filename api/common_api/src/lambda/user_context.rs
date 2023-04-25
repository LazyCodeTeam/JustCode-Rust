use common_domain::error::{Error, Result};
use lambda_http::{request::RequestContext, Request, RequestExt};

const USER_ID_KEY: &str = "sub";
const USERNAME_KEY: &str = "username";

pub trait UserContext {
    fn get_user_id(&self) -> Result<String>;

    fn get_username(&self) -> Result<String>;
}

impl UserContext for Request {
    fn get_user_id(&self) -> Result<String> {
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
                .ok_or_else(|| Error::unknown("User ID not found in request context".to_owned())),
        }
    }

    fn get_username(&self) -> Result<String> {
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
                .ok_or_else(|| Error::unknown("Username not found in request context".to_owned())),
        }
    }
}
