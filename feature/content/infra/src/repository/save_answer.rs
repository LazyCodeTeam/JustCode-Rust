use common_domain::error::{Result, ResultLogExt};
use common_infra::dynamodb::client::get_dynamodb_client;
use content_domain::model::answer_to_save::AnswerToSave;
use serde_dynamo::to_item;
use snafu::ResultExt;

use crate::{config::CONFIG, historical_answer_dto::HistoricalAnswerDto, MapFrom};

pub async fn save_answer(answer: AnswerToSave) -> Result<()> {
    let item = to_item(HistoricalAnswerDto::map_from(answer))
        .whatever_context("Failed to serialize answer")
        .with_error_log()?;

    get_dynamodb_client()
        .await
        .put_item()
        .table_name(&CONFIG.dynamodb_table)
        .set_item(Some(item))
        .send()
        .await
        .map(|_| ())
        .whatever_context("Failed to save answer")
        .with_error_log()
}
