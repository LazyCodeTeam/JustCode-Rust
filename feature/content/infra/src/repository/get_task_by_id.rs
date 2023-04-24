use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::task::Task;

use crate::{config::CONFIG, task_dto::TaskDto, IntoModel, TASK_GSI_PK, TASK_ID_PREFIX};

pub async fn get_task_by_id(task_id: impl Into<String>) -> Result<Option<Task>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.content_dynamodb_table)
        .index_name("GSI_1")
        .key_condition_expression("GSI_1_PK = :gsi_1_pk and GSI_1_SK = :gsi_1_sk")
        .expression_attribute_values(":gsi_1_pk", AttributeValue::S(TASK_GSI_PK.to_owned()))
        .expression_attribute_values(
            ":gsi_1_sk",
            AttributeValue::S(format!("{}{}", TASK_ID_PREFIX, task_id.into())),
        )
        .send()
        .await
        .parse_one::<TaskDto>()
        .map(|o| o.map(IntoModel::into_model))
}
