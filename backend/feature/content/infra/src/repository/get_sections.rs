use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::section::Section;

use crate::{
    config::CONFIG, section_dto::SectionDto, IntoModel, SECTION_ID_PREFIX, TECHNOLOGY_ID_PREFIX,
};

pub async fn get_sections_for_multiple_technologies(ids: Vec<String>) -> Result<Vec<Section>> {
    let mut sections = Vec::new();

    for id in ids {
        let technology_sections = get_all_technology_sections(&id).await?;
        sections.extend(technology_sections);
    }

    Ok(sections)
}

pub async fn get_all_technology_sections(technology_id: &str) -> Result<Vec<Section>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk and begins_with(SK, :sk)")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", TECHNOLOGY_ID_PREFIX, technology_id)),
        )
        .expression_attribute_values(":sk", AttributeValue::S(SECTION_ID_PREFIX.to_string()))
        .send()
        .await
        .parse::<SectionDto>()
        .map(IntoModel::into_model)
}

pub async fn get_ordered_technology_sections(technology_id: &str) -> Result<Vec<Section>> {
    get_dynamodb_client()
        .await
        .query()
        .index_name("LSI_1")
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk and begins_with(LSI_1, :lsi_1)")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", TECHNOLOGY_ID_PREFIX, technology_id)),
        )
        .expression_attribute_values(":lsi_1", AttributeValue::S(SECTION_ID_PREFIX.to_string()))
        .send()
        .await
        .parse::<SectionDto>()
        .map(IntoModel::into_model)
}
