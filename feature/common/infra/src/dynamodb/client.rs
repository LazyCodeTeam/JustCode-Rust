use aws_sdk_dynamodb::{
    operation::{
        get_item::{GetItemError, GetItemOutput},
        query::{QueryError, QueryOutput},
    },
    Client,
};
use aws_smithy_http::result::SdkError;
use common_domain::error::{Result, ResultLogExt};
use serde_dynamo::{from_item, from_items};
use snafu::{OptionExt, ResultExt};

pub async fn get_dynamodb_client() -> &'static Client {
    static CLIENT: tokio::sync::OnceCell<Client> = tokio::sync::OnceCell::const_new();

    CLIENT
        .get_or_init(|| async {
            log::debug!("DynamoDB is initializing...");

            let config = aws_config::load_from_env().await;
            let client = aws_sdk_dynamodb::Client::new(&config);
            log::debug!("DynamoDB is initialized.");

            client
        })
        .await
}

pub trait GetItemOutputExt {
    fn parse<'a, T>(self) -> Result<Option<T>>
    where
        T: serde::Deserialize<'a>;
}

pub trait QueryOutputExt {
    fn parse<'a, T>(self) -> Result<Vec<T>>
    where
        T: serde::Deserialize<'a>;

    fn parse_one<'a, T>(self) -> Result<Option<T>>
    where
        T: serde::Deserialize<'a>;
}

impl GetItemOutputExt for std::result::Result<GetItemOutput, SdkError<GetItemError>> {
    fn parse<'a, T>(self) -> Result<Option<T>>
    where
        T: serde::Deserialize<'a>,
    {
        self.map(|result| result.item)
            .whatever_context("Failed to get item from DynamoDB")
            .with_error_log()
            .and_then(|item| match item {
                Some(item) => from_item::<_, T>(item)
                    .map(Some)
                    .whatever_context("Failed to parse item from DynamoDB")
                    .with_error_log(),
                None => Ok(None),
            })
    }
}

impl QueryOutputExt for std::result::Result<QueryOutput, SdkError<QueryError>> {
    fn parse<'a, T>(self) -> Result<Vec<T>>
    where
        T: serde::Deserialize<'a>,
    {
        self.whatever_context("Failed to query DynamoDB")
            .map(|output| output.items)
            .and_then(|items| {
                items
                    .whatever_context("Failed to get items from DynamoDB")
                    .and_then(|items| {
                        from_items::<_, T>(items)
                            .whatever_context("Failed to parse items from DynamoDB")
                    })
            })
            .with_error_log()
    }

    fn parse_one<'a, T>(self) -> Result<Option<T>>
    where
        T: serde::Deserialize<'a>,
    {
        let result: Option<_> = self
            .whatever_context("Failed to query single item from DynamoDB")
            .map(|output| output.items.and_then(|items| items.into_iter().next()))
            .with_error_log()?;

        match result {
            Some(item) => from_item(item).whatever_context("Failed to parse item from DynamoDB"),
            _ => Ok(None),
        }
        .with_error_log()
    }
}
