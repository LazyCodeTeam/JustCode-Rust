use std::{env, panic::AssertUnwindSafe};

use aws_sdk_dynamodb::{
    types::{
        AttributeDefinition, GlobalSecondaryIndex, KeySchemaElement, KeyType, Projection,
        ProvisionedThroughput, ScalarAttributeType, TableStatus,
    },
    Client,
};
use futures::FutureExt;

use super::Error;

pub async fn create_table(client: &Client, table: &str) -> Result<(), Error> {
    let pk_ad = AttributeDefinition::builder()
        .attribute_name("PK")
        .attribute_type(ScalarAttributeType::S)
        .build();
    let pk_ks = KeySchemaElement::builder()
        .attribute_name("PK")
        .key_type(KeyType::Hash)
        .build();

    let sk_ad = AttributeDefinition::builder()
        .attribute_name("SK")
        .attribute_type(ScalarAttributeType::S)
        .build();
    let sk_ks = KeySchemaElement::builder()
        .attribute_name("SK")
        .key_type(KeyType::Range)
        .build();

    let gsi_pk_ad = AttributeDefinition::builder()
        .attribute_name("GSI_PK")
        .attribute_type(ScalarAttributeType::S)
        .build();
    let gsi_pk_ks = KeySchemaElement::builder()
        .attribute_name("GSI_PK")
        .key_type(KeyType::Hash)
        .build();

    let gsi_sk_ad = AttributeDefinition::builder()
        .attribute_name("GSI_SK")
        .attribute_type(ScalarAttributeType::S)
        .build();
    let gsi_sk_ks = KeySchemaElement::builder()
        .attribute_name("GSI_SK")
        .key_type(KeyType::Range)
        .build();

    let gsi = GlobalSecondaryIndex::builder()
        .index_name("GSI")
        .key_schema(gsi_pk_ks)
        .key_schema(gsi_sk_ks)
        .projection(
            Projection::builder()
                .projection_type(aws_sdk_dynamodb::types::ProjectionType::All)
                .build(),
        )
        .provisioned_throughput(
            ProvisionedThroughput::builder()
                .read_capacity_units(1)
                .write_capacity_units(1)
                .build(),
        )
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build();

    client
        .create_table()
        .table_name(table)
        .key_schema(pk_ks)
        .key_schema(sk_ks)
        .attribute_definitions(pk_ad)
        .attribute_definitions(sk_ad)
        .attribute_definitions(gsi_pk_ad)
        .attribute_definitions(gsi_sk_ad)
        .provisioned_throughput(pt)
        .global_secondary_indexes(gsi)
        .send()
        .await?;

    Ok(())
}

pub async fn delete_table(client: &Client, table: &str) -> Result<(), Error> {
    client.delete_table().table_name(table).send().await?;

    Ok(())
}

async fn wait_until_created(client: &Client, table: &str) -> Result<(), Error> {
    let mut rt = tokio::time::interval(std::time::Duration::from_secs(5));
    let mut limit = tokio::time::interval(std::time::Duration::from_secs(60));
    rt.tick().await;
    limit.tick().await;

    loop {
        tokio::select! {
            _ = rt.tick() => {
                let resp = client.describe_table().table_name(table).send().await?;
                if let Some(TableStatus::Active) = resp.table.and_then(|t| t.table_status) {
                    return Ok(());
                }
            }
            _ = limit.tick() => {
                Err("Timeout")?
            }
        }
    }
}

async fn await_until_destroyed(client: &Client, table: &str) -> Result<(), Error> {
    let mut rt = tokio::time::interval(std::time::Duration::from_secs(5));
    let mut limit = tokio::time::interval(std::time::Duration::from_secs(60));
    rt.tick().await;
    limit.tick().await;

    loop {
        tokio::select! {
            _ = rt.tick() => {
                let table = client.describe_table().table_name(table).send().await.ok().and_then(|r| r.table);
                if table.is_none() {
                    return Ok(());
                }
            }
            _ = limit.tick() => {
                Err("Timeout")?
            }
        }
    }
}

lazy_static::lazy_static! {
    static ref DYNAMODB_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::new(());
}

pub async fn with_table<'a, F, FUT, R>(client: &Client, f: F) -> R
where
    F: FnOnce(&'a str) -> FUT,
    FUT: std::future::Future<Output = R>,
{
    let _lock = DYNAMODB_LOCK.lock().await;
    let table = "just-code-test-table";
    create_table(client, table)
        .await
        .expect("Failed to create table");
    wait_until_created(client, table)
        .await
        .expect("Failed to wait for table to be created");
    env::set_var("DYNAMODB_TABLE", table);
    let result = AssertUnwindSafe(f(table)).catch_unwind().await;
    delete_table(client, table)
        .await
        .expect("Failed to delete table");
    await_until_destroyed(client, table)
        .await
        .expect("Failed to wait for table to be destroyed");

    match result {
        Ok(r) => r,
        Err(e) => std::panic::resume_unwind(e),
    }
}
