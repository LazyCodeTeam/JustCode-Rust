use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::historical_answer::HistoricalAnswer;
use serde_dynamo::from_items;

use crate::{
    answer_result_dto::AnswerResultDto, config::CONFIG, historical_answer_dto::HistoricalAnswerDto,
    USER_ANSWER_ID_PREFIX,
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
        .map_err(|e| Error::unknown(format!("Failed to get previous answers: {e:?}")))
        .and_then(|r| {
            r.items
                .ok_or_else(|| {
                    Error::unknown("Failed to get previous answers - option is empty".to_owned())
                })
                .and_then(|items| {
                    from_items::<_, HistoricalAnswerDto>(items)
                        .map_err(|e| {
                            Error::unknown(format!("Failed to parse historical answers: {e:?}"))
                        })
                        .map(|dtos| {
                            dtos.into_iter()
                                .map(HistoricalAnswer::from)
                                .collect::<Vec<_>>()
                        })
                })
        })
}
