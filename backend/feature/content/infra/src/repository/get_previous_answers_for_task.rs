use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::historical_answer::HistoricalAnswer;

use crate::{
    config::CONFIG, historical_answer_dto::HistoricalAnswerDto, IntoModel, TASK_ID_PREFIX,
    USER_ANSWER_ID_PREFIX,
};

pub async fn get_previous_answers_for_task(
    user_id: String,
    task_id: String,
) -> Result<Vec<HistoricalAnswer>> {
    get_dynamodb_client()
        .await
        .query()
        .index_name("LSI_1")
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk AND LSI_1 = :lsi_1")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", USER_ANSWER_ID_PREFIX, user_id)),
        )
        .expression_attribute_values(
            ":lsi_1",
            AttributeValue::S(format!("{}{}", TASK_ID_PREFIX, task_id)),
        )
        .send()
        .await
        .parse::<HistoricalAnswerDto>()
        .map(IntoModel::into_model)
}
