use std::collections::HashMap;

use crate::common::dynamodb::with_table;
use aws_lambda_events::cognito::{
    CognitoEventUserPoolsPostConfirmationRequest, CognitoEventUserPoolsPostConfirmationResponse,
};
use common_infra::dynamodb_client::get_dynamodb_client;
use lambda_http::{
    aws_lambda_events::apigw::ApiGatewayProxyRequestContext, request::RequestContext, Body,
    Context, Response,
};
use serde_json::Value;

#[tokio::test]
#[ignore]
async fn create_profile_test() {
    let client = get_dynamodb_client().await;

    with_table(client, run_test).await;
}

async fn run_test() {
    let id = "123";
    let email = "example@gmail.com";
    let username = "example";
    let result = get_user(id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status(), 404);

    create_profile(id, username, email)
        .await
        .expect("Failed to create profile");

    let result = get_user(id).await;
    assert!(result.is_ok());
    assert_eq!(result.as_ref().unwrap().status(), 200);
    let body = serde_json::from_slice::<HashMap<String, Value>>(result.unwrap().body())
        .expect("Failed to parse body");
    assert_eq!(body.get("id").unwrap(), id);
    assert_eq!(body.get("email").unwrap(), email);
    assert_eq!(body.get("name").unwrap(), username);
}

async fn create_profile(id: &str, name: &str, email: &str) -> Result<(), lambda_runtime::Error> {
    let event = lambda_runtime::LambdaEvent {
        payload: aws_lambda_events::cognito::CognitoEventUserPoolsPostConfirmation {
            response: CognitoEventUserPoolsPostConfirmationResponse {},
            request: CognitoEventUserPoolsPostConfirmationRequest {
                user_attributes: HashMap::from([
                    ("email".to_owned(), email.to_owned()),
                    ("sub".to_owned(), id.to_owned()),
                ]),
                client_metadata: HashMap::new(),
            },
            cognito_event_user_pools_header:
                aws_lambda_events::cognito::CognitoEventUserPoolsHeader {
                    user_name: Some(name.to_owned()),
                    ..Default::default()
                },
        },
        context: Context::default(),
    };

    create_profile::handle_request(event).await?;

    Ok(())
}

async fn get_user(id: &str) -> Result<Response<Body>, lambda_http::Error> {
    let request = lambda_http::http::Request::builder()
        .method("GET")
        .uri("/user")
        .extension(RequestContext::ApiGatewayV1(
            ApiGatewayProxyRequestContext {
                authorizer: HashMap::from([(
                    "claims".to_string(),
                    serde_json::json!({
                        "sub": id,
                    }),
                )]),

                ..Default::default()
            },
        ))
        .body(lambda_http::Body::Empty)
        .unwrap();

    get_profile_v1::handle_request(request).await
}
