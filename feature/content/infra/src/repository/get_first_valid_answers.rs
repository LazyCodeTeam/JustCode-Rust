use aws_sdk_dynamodb::types::AttributeValue;
use common_domain::error::Result;
use common_infra::dynamodb_client::{get_dynamodb_client, QueryOutputExt};
use content_domain::model::historical_answer::HistoricalAnswer;

use crate::{
    answer_result_dto::AnswerResultDto, config::CONFIG, historical_answer_dto::HistoricalAnswerDto,
    IntoModel, USER_ANSWER_ID_PREFIX,
};

pub async fn get_first_valid_answers(user_id: String) -> Result<Vec<HistoricalAnswer>> {
    get_dynamodb_client()
        .await
        .query()
        .index_name("LSI_2")
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :pk AND LSI_2 = :lsi_2")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S(format!("{}{}", USER_ANSWER_ID_PREFIX, user_id)),
        )
        .expression_attribute_values(
            ":lsi_2",
            AttributeValue::S(AnswerResultDto::Valid.to_string()),
        )
        .send()
        .await
        .parse::<HistoricalAnswerDto>()
        .map(IntoModel::into_model)
}
