use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::section::Section;
use serde_dynamo::from_items;

use crate::{config::CONFIG, section_dto::SectionDto, SECTION_ID_PREFIX, TECHNOLOGY_ID_PREFIX};

pub async fn get_sections_for_multiple_technologies(ids: Vec<String>) -> Result<Vec<Section>> {
    let mut sections = Vec::new();

    for id in ids {
        let technology_sections = get_technology_sections(&id).await?;
        sections.extend(technology_sections);
    }

    Ok(sections)
}

pub async fn get_technology_sections(technology_id: &str) -> Result<Vec<Section>> {
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
        .map_err(|e| Error::unknown(format!("Failed to get sections: {e:?}")))
        .and_then(|r| {
            r.items
                .ok_or_else(|| {
                    Error::unknown("Failed to get sections - option is empty".to_owned())
                })
                .and_then(|items| {
                    from_items::<_, SectionDto>(items)
                        .map_err(|e| Error::unknown(format!("Failed to parse sections: {e:?}")))
                        .map(|dtos| dtos.into_iter().map(Into::into).collect())
                })
        })
}
