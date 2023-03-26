use chrono::{DateTime, Utc};
use content_domain::model::{answer_to_save::AnswerToSave, historical_answer::HistoricalAnswer};
use serde::{Deserialize, Serialize};

use crate::{answer_result_dto::AnswerResultDto, TASK_ID_PREFIX, USER_ANSWER_ID_PREFIX};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalAnswerDto {
    #[serde(rename = "PK")]
    pub user_id: String,
    #[serde(rename = "SK")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "LSI_1")]
    pub task_id: String,
    #[serde(rename = "LSI_2")]
    pub result: AnswerResultDto,
}

impl From<AnswerToSave> for HistoricalAnswerDto {
    fn from(value: AnswerToSave) -> Self {
        Self {
            user_id: format!("{}{}", USER_ANSWER_ID_PREFIX, value.user_id),
            task_id: format!("{}{}", TASK_ID_PREFIX, value.answer.task_id),
            result: value.result.into(),
            created_at: Utc::now(),
        }
    }
}

impl From<HistoricalAnswerDto> for HistoricalAnswer {
    fn from(value: HistoricalAnswerDto) -> Self {
        Self {
            user_id: value.user_id.replace(USER_ANSWER_ID_PREFIX, ""),
            task_id: value.task_id.replace(TASK_ID_PREFIX, ""),
            result: value.result.into(),
            created_at: value.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::{
        answer::Answer, answer_content::AnswerContent, answer_result::AnswerResult,
    };

    use super::*;

    #[test]
    fn from_answer_to_save() {
        let answer_to_save = AnswerToSave {
            user_id: "user_id".to_string(),
            answer: Answer {
                task_id: "task_id".to_string(),
                content: AnswerContent::Empty,
            },
            result: AnswerResult::Valid,
        };

        let before = Utc::now();
        let historical_answer_dto = HistoricalAnswerDto::from(answer_to_save);
        let after = Utc::now();

        assert_eq!(
            historical_answer_dto.user_id,
            format!("{}{}", USER_ANSWER_ID_PREFIX, "user_id")
        );
        assert_eq!(
            historical_answer_dto.task_id,
            format!("{}{}", TASK_ID_PREFIX, "task_id")
        );
        assert_eq!(historical_answer_dto.result, AnswerResultDto::Valid);
        assert!(historical_answer_dto.created_at >= before);
        assert!(historical_answer_dto.created_at <= after);
    }

    #[test]
    fn try_from_historical_answer_dto() {
        let now = Utc::now();
        let historical_answer_dto = HistoricalAnswerDto {
            user_id: format!("{}{}", USER_ANSWER_ID_PREFIX, "user_id"),
            task_id: format!("{}{}", TASK_ID_PREFIX, "task_id"),
            result: AnswerResultDto::Valid,
            created_at: now,
        };

        let historical_answer = HistoricalAnswer::from(historical_answer_dto);

        assert_eq!(historical_answer.user_id, "user_id");
        assert_eq!(historical_answer.task_id, "task_id");
        assert_eq!(historical_answer.result, AnswerResultDto::Valid.into());
        assert_eq!(historical_answer.created_at, now);
    }
}
