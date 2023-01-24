use aws_sdk_dynamodb::{
    model::{
        AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
    },
    Client,
};

use super::Error;

pub async fn create_table(client: &Client, table: &str) -> Result<(), Error> {
    let ad = AttributeDefinition::builder()
        .attribute_name("PK")
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name("PK")
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(1)
        .write_capacity_units(1)
        .build();

    client
        .create_table()
        .table_name(table)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await?;

    Ok(())
}

pub async fn delete_table(client: &Client, table: &str) -> Result<(), Error> {
    client.delete_table().table_name(table).send().await?;

    Ok(())
}
