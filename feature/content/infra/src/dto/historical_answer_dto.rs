use chrono::{DateTime, Utc};
use content_domain::model::{answer_to_save::AnswerToSave, historical_answer::HistoricalAnswer};
use serde::{Deserialize, Serialize};

use crate::{
    answer_result_dto::AnswerResultDto, MapFrom, MapInto, TASK_ID_PREFIX, USER_ANSWER_ID_PREFIX,
};

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

impl MapFrom<AnswerToSave> for HistoricalAnswerDto {
    fn map_from(model: AnswerToSave) -> Self {
        Self {
            user_id: format!("{}{}", USER_ANSWER_ID_PREFIX, model.user_id),
            task_id: format!("{}{}", TASK_ID_PREFIX, model.answer.task_id),
            result: model.result.map_into(),
            created_at: Utc::now(),
        }
    }
}

impl MapFrom<HistoricalAnswerDto> for HistoricalAnswer {
    fn map_from(dto: HistoricalAnswerDto) -> Self {
        Self {
            user_id: dto.user_id.replace(USER_ANSWER_ID_PREFIX, ""),
            task_id: dto.task_id.replace(TASK_ID_PREFIX, ""),
            result: dto.result.map_into(),
            created_at: dto.created_at,
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
        let historical_answer_dto = HistoricalAnswerDto::map_from(answer_to_save);
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

        let historical_answer = HistoricalAnswer::map_from(historical_answer_dto);

        assert_eq!(historical_answer.user_id, "user_id");
        assert_eq!(historical_answer.task_id, "task_id");
        assert_eq!(historical_answer.result, AnswerResultDto::Valid.map_into());
        assert_eq!(historical_answer.created_at, now);
    }
}
