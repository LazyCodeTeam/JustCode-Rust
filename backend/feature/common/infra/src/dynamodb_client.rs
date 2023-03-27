use aws_sdk_dynamodb::{
    error::{GetItemError, QueryError},
    output::{GetItemOutput, QueryOutput},
    types::SdkError,
    Client,
};
use common_domain::error::{Error, Result};
use serde_dynamo::{from_item, from_items};

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
            .map_err(|e| Error::unknown(format!("Failed to get item: {e:?}")))
            .and_then(|item| match item {
                Some(item) => from_item::<_, T>(item).map(Some).map_err(|e| {
                    Error::unknown(format!(
                        "Failed to parse item {}: {:?}",
                        std::any::type_name::<T>(),
                        e
                    ))
                }),
                None => Ok(None),
            })
    }
}

impl QueryOutputExt for std::result::Result<QueryOutput, SdkError<QueryError>> {
    fn parse<'a, T>(self) -> Result<Vec<T>>
    where
        T: serde::Deserialize<'a>,
    {
        self.map_err(|e| Error::unknown(format!("Failed preform query: {e:?}")))
            .map(|output| output.items)
            .and_then(|items| {
                items
                    .ok_or_else(|| {
                        Error::unknown("Failed to parse query result - empty option".to_owned())
                    })
                    .and_then(|items| {
                        from_items::<_, T>(items).map_err(|e| {
                            Error::unknown(format!(
                                "Failed to parse output {}: {:?}",
                                std::any::type_name::<T>(),
                                e
                            ))
                        })
                    })
            })
    }

    fn parse_one<'a, T>(self) -> Result<Option<T>>
    where
        T: serde::Deserialize<'a>,
    {
        let result: Option<_> = self
            .map_err(|e| Error::unknown(format!("Failed preform query: {e:?}")))
            .map(|output| output.items.and_then(|items| items.into_iter().next()))?;

        match result {
            Some(item) => from_item(item).map_err(|e| {
                Error::unknown(format!(
                    "Failed to parse output {}: {:?}",
                    std::any::type_name::<T>(),
                    e
                ))
            }),
            _ => Ok(None),
        }
    }
}
