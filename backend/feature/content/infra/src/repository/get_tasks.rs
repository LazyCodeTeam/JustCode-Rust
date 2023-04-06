use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::task::Task;

use crate::{config::CONFIG, task_dto::TaskDto, IntoModel, SECTION_ID_PREFIX, TASK_ID_PREFIX};

pub async fn get_tasks_for_multiple_technologies(ids: Vec<String>) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();

    for id in ids {
        let technology_sections = get_all_section_tasks(&id).await?;
        tasks.extend(technology_sections);
    }

    Ok(tasks)
}

pub async fn get_all_section_tasks(section_id: &str) -> Result<Vec<Task>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.content_dynamodb_table)
        .key_condition_expression("PK = :pk and begins_with(SK, :sk)")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", SECTION_ID_PREFIX, section_id)),
        )
        .expression_attribute_values(":sk", AttributeValue::S(TASK_ID_PREFIX.to_string()))
        .send()
        .await
        .parse::<TaskDto>()
        .map(IntoModel::into_model)
}

pub async fn get_ordered_section_tasks(section_id: String) -> Result<Vec<Task>> {
    get_dynamodb_client()
        .await
        .query()
        .index_name("LSI_1")
        .table_name(&CONFIG.content_dynamodb_table)
        .key_condition_expression("PK = :pk and begins_with(LSI_1, :lsi_1)")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", SECTION_ID_PREFIX, section_id)),
        )
        .expression_attribute_values(":lsi_1", AttributeValue::S(TASK_ID_PREFIX.to_string()))
        .send()
        .await
        .parse::<TaskDto>()
        .map(IntoModel::into_model)
}
