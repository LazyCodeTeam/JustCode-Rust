use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb::client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::technology::Technology;

use crate::{
    config::CONFIG, dto::technology_dto::TechnologyDto, MapInto, TECHNOLOGY_ID_PREFIX,
    TECHNOLOGY_PK,
};

pub async fn get_all_technologies() -> Result<Vec<Technology>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.content_dynamodb_table)
        .key_condition_expression("PK = :pk and begins_with(SK, :sk)")
        .expression_attribute_values(":pk", AttributeValue::S(TECHNOLOGY_PK.to_string()))
        .expression_attribute_values(":sk", AttributeValue::S(TECHNOLOGY_ID_PREFIX.to_string()))
        .send()
        .await
        .parse::<TechnologyDto>()
        .map(MapInto::map_into)
}

pub async fn get_ordered_technologies() -> Result<Vec<Technology>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.content_dynamodb_table)
        .index_name("LSI_1")
        .key_condition_expression("PK = :pk and begins_with(LSI_1, :lsi_1)")
        .expression_attribute_values(":pk", AttributeValue::S(TECHNOLOGY_PK.to_string()))
        .expression_attribute_values(
            ":lsi_1",
            AttributeValue::S(TECHNOLOGY_ID_PREFIX.to_string()),
        )
        .send()
        .await
        .parse::<TechnologyDto>()
        .map(MapInto::map_into)
}
