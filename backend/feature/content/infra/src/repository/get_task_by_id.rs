use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::task::Task;
use serde_dynamo::from_items;

use crate::{config::CONFIG, task_dto::TaskDto, TASK_GSI_PK, TASK_ID_PREFIX};

pub async fn get_task_by_id(task_id: impl Into<String>) -> Result<Option<Task>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .index_name("GSI_1")
        .key_condition_expression("GSI_1_PK = :GSI_1_PK and GSI_1_SK = :GSI_1_SK")
        .expression_attribute_values("GSI_1_PK", AttributeValue::S(TASK_GSI_PK.to_owned()))
        .expression_attribute_values(
            "GSI_1_SK",
            AttributeValue::S(format!("{}{}", TASK_ID_PREFIX, task_id.into())),
        )
        .send()
        .await
        .map_err(|e| Error::unknown(format!("Failed to get task by id: {e:?}")))
        .and_then(|r| {
            r.items
                .ok_or_else(|| {
                    Error::unknown("Failed to get task by id - option is empty".to_owned())
                })
                .and_then(|items| {
                    from_items::<_, TaskDto>(items)
                        .map_err(|e| Error::unknown(format!("Failed to parse tasks: {e:?}")))
                        .map(|dtos| dtos.into_iter().map(Into::into).next())
                })
        })
}
