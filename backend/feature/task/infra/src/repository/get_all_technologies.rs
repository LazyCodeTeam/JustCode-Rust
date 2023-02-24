use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use serde_dynamo::from_items;
use task_domain::model::technology::Technology;

use crate::{config::CONFIG, dto::technology_dto::TechnologyDto, TECHNOLOGY_PK};

pub async fn get_all_technologies() -> Result<Vec<Technology>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk")
        .expression_attribute_values(":pk", AttributeValue::S(TECHNOLOGY_PK.to_string()))
        .send()
        .await
        .map_err(|e| Error::unknown(format!("Failed to get technologies: {e:?}")))
        .and_then(|r| {
            r.items
                .ok_or_else(|| {
                    Error::unknown("Failed to get technologies - option is empty".to_owned())
                })
                .and_then(|items| {
                    from_items::<_, TechnologyDto>(items)
                        .map_err(|e| Error::unknown(format!("Failed to parse technologies: {e:?}")))
                        .map(|dtos| dtos.into_iter().map(Into::into).collect())
                })
        })
}
