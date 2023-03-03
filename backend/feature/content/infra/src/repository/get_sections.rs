use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::section::Section;
use serde_dynamo::from_items;

use crate::{config::CONFIG, section_dto::SectionDto, TECHNOLOGY_ID_PREFIX};

pub async fn get_sections_for_multiple_technologies(ids: Vec<String>) -> Result<Vec<Section>> {
    let mut sections = Vec::new();

    for id in ids {
        let technology_sections = get_technology_sections(id).await?;
        sections.extend(technology_sections);
    }

    Ok(sections)
}

pub async fn get_technology_sections(technology_id: String) -> Result<Vec<Section>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", TECHNOLOGY_ID_PREFIX, technology_id)),
        )
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
