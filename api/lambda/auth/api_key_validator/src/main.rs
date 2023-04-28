use std::env;

use aws_lambda_events::apigw::{
    ApiGatewayV2CustomAuthorizerSimpleResponse, ApiGatewayV2CustomAuthorizerV2Request,
};
use common_api::lambda::register_internal_handler::register_internal_handler;
use lambda_runtime::{Error, LambdaEvent};
use use_case::auth::validate_secret_key::validate_secret_key;

#[tokio::main]
async fn main() -> Result<(), Error> {
    register_internal_handler(handle_request).await
}

async fn handle_request(
    event: LambdaEvent<ApiGatewayV2CustomAuthorizerV2Request>,
) -> Result<ApiGatewayV2CustomAuthorizerSimpleResponse, lambda_runtime::Error> {
    let key = event.payload.identity_source.first().map(AsRef::as_ref);
    let expected_key = env::var("API_KEY").ok();

    let result = validate_secret_key(key, expected_key.as_deref()).await;

    Ok(ApiGatewayV2CustomAuthorizerSimpleResponse {
        is_authorized: result.is_ok(),
        context: serde_json::Value::Object(serde_json::Map::new()),
    })
}
