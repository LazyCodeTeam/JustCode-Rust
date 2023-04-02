use aws_sdk_cognitoidentityprovider::Client;

pub async fn get_cognito_client() -> &'static Client {
    static CLIENT: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

    CLIENT
        .get_or_init(|| async {
            log::debug!("Cognito client is initializing...");

            let config = aws_config::load_from_env().await;
            let client = Client::new(&config);
            log::debug!("Cognito client is initialized.");

            client
        })
        .await
}
