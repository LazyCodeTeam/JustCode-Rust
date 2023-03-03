use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::task::Task;
use serde_dynamo::from_items;

use crate::{config::CONFIG, task_dto::TaskDto, SECTION_ID_PREFIX};

pub async fn get_tasks_for_multiple_technologies(ids: Vec<String>) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for id in ids {
        let technology_sections = get_section_tasks(id).await?;
        tasks.extend(technology_sections);
    }

    Ok(tasks)
}

pub async fn get_section_tasks(section_id: String) -> Result<Vec<Task>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", SECTION_ID_PREFIX, section_id)),
        )
        .send()
        .await
        .map_err(|e| Error::unknown(format!("Failed to get tasks: {e:?}")))
        .and_then(|r| {
            r.items
                .ok_or_else(|| Error::unknown("Failed to get tasks - option is empty".to_owned()))
                .and_then(|items| {
                    from_items::<_, TaskDto>(items)
                        .map_err(|e| Error::unknown(format!("Failed to parse tasks: {e:?}")))
                        .map(|dtos| dtos.into_iter().map(Into::into).collect())
                })
        })
}
