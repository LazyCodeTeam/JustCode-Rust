use aws_sdk_dynamodb::model::AttributeValue;
use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::historical_answer::HistoricalAnswer;
use serde_dynamo::from_items;

use crate::{
    config::CONFIG, historical_answer_dto::HistoricalAnswerDto, ANSWER_ID_PREFIX,
    USER_ANSWER_ID_PREFIX,
};

pub async fn get_previous_answers_for_task(
    user_id: String,
    task_id: String,
) -> Result<Vec<HistoricalAnswer>> {
    get_dynamodb_client()
        .await
        .query()
        .table_name(&CONFIG.dynamodb_table)
        .key_condition_expression("PK = :PK AND begins_with(SK, :SK)")
        .expression_attribute_values(
            "PK",
            AttributeValue::S(format!("{}{}", USER_ANSWER_ID_PREFIX, user_id)),
        )
        .expression_attribute_values(
            "SK",
            AttributeValue::S(format!("{}{}", ANSWER_ID_PREFIX, task_id)),
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
                        .and_then(|dtos| {
                            dtos.into_iter()
                                .map(HistoricalAnswer::try_from)
                                .collect::<Result<Vec<_>>>()
                        })
                })
        })
}
