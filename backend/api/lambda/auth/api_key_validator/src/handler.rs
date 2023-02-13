use aws_lambda_events::{
    apigw::{ApiGatewayV2CustomAuthorizerSimpleResponse, ApiGatewayV2CustomAuthorizerV2Request},
    serde_json::Map,
};
use lambda_runtime::{Error, LambdaEvent};

pub async fn handle_request(
    _event: LambdaEvent<ApiGatewayV2CustomAuthorizerV2Request>,
) -> Result<ApiGatewayV2CustomAuthorizerSimpleResponse, Error> {
    log::error!("Event: {_event:?}");

    Ok(ApiGatewayV2CustomAuthorizerSimpleResponse {
        is_authorized: true,
        context: aws_lambda_events::serde_json::Value::Object(Map::new()),
    })
}
