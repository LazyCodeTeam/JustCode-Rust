use common_domain::error::{Error, Result};
use common_infra::dynamodb_client::get_dynamodb_client;
use content_domain::model::answer_to_save::AnswerToSave;
use serde_dynamo::to_item;

use crate::{config::CONFIG, historical_answer_dto::HistoricalAnswerDto};

pub async fn save_answer(answer: AnswerToSave) -> Result<()> {
    let item = to_item(HistoricalAnswerDto::from(answer))
        .map_err(|e| Error::unknown(format!("Failed to serialize answer ({e:?})")))?;

    get_dynamodb_client()
        .await
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map_err(|e| Error::unknown(format!("Failed to save answer: {e:?}")))
        .map(|_| ())
}
