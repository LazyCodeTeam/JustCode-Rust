use std::env;

use aws_lambda_events::{
    apigw::{ApiGatewayV2CustomAuthorizerSimpleResponse, ApiGatewayV2CustomAuthorizerV2Request},
    serde_json::Map,
};
use lambda_runtime::LambdaEvent;
use use_case::auth::validate_secret_key::validate_secret_key;

pub async fn handle_request(
    event: LambdaEvent<ApiGatewayV2CustomAuthorizerV2Request>,
) -> Result<ApiGatewayV2CustomAuthorizerSimpleResponse, lambda_runtime::Error> {
    let key = event.payload.identity_source.first().map(AsRef::as_ref);
    let expected_key = env::var("API_KEY").ok();

    let result = validate_secret_key(key, expected_key.as_deref()).await;

    if let Err(ref err) = result {
        err.log();
    }

    Ok(ApiGatewayV2CustomAuthorizerSimpleResponse {
        is_authorized: result.is_ok(),
        context: aws_lambda_events::serde_json::Value::Object(Map::new()),
    })
}
