use chrono::{DateTime, Utc};
use common_domain::error::Error;
use content_domain::model::{answer_to_save::AnswerToSave, historical_answer::HistoricalAnswer};
use serde::{Deserialize, Serialize};

use crate::{ANSWER_ID_PREFIX, USER_ANSWER_ID_PREFIX};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HistoricalAnswerDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub user_id: String,
    pub task_id: String,
    pub was_valid: bool,
    #[serde(rename = "LSI_1")]
    pub date: DateTime<Utc>,
}

impl From<AnswerToSave> for HistoricalAnswerDto {
    fn from(value: AnswerToSave) -> Self {
        Self {
            id: format!(
                "{}{}#{}",
                ANSWER_ID_PREFIX,
                value.answer.task_id,
                uuid::Uuid::new_v4().simple()
            ),
            user_id: format!("{}{}", USER_ANSWER_ID_PREFIX, value.user_id),
            task_id: value.answer.task_id,
            was_valid: value.is_valid,
            date: Utc::now(),
        }
    }
}

impl TryFrom<HistoricalAnswerDto> for HistoricalAnswer {
    type Error = Error;

    fn try_from(value: HistoricalAnswerDto) -> Result<Self, Self::Error> {
        let id = value
            .id
            .split('#')
            .last()
            .ok_or_else(|| Error::unknown("Invalid historical answer id format"))?
            .to_string();
        let user_id = value.user_id.replace(USER_ANSWER_ID_PREFIX, "");

        Ok(Self {
            id,
            user_id,
            task_id: value.task_id,
            was_valid: value.was_valid,
            date: value.date,
        })
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::answer::Answer;

    use super::*;

    #[test]
    fn from_answer_to_save() {
        let answer_to_save = AnswerToSave {
            user_id: "user_id".to_string(),
            is_valid: true,
            had_valid_answer_before: false,
            answer: Answer {
                task_id: "task_id".to_string(),
                ..Default::default()
            },
        };

        let historical_answer_dto = HistoricalAnswerDto::from(answer_to_save);

        assert!(historical_answer_dto
            .id
            .starts_with(format!("{}{}#", ANSWER_ID_PREFIX, "task_id").as_str()));
        assert_eq!(
            historical_answer_dto.user_id,
            format!("{}{}", USER_ANSWER_ID_PREFIX, "user_id")
        );
        assert_eq!(historical_answer_dto.task_id, "task_id");
        assert!(historical_answer_dto.was_valid);
    }

    #[test]
    fn try_from_historical_answer_dto() {
        let now = Utc::now();
        let historical_answer_dto = HistoricalAnswerDto {
            id: "task_id#id".to_string(),
            user_id: "user_id".to_string(),
            task_id: "task_id".to_string(),
            was_valid: true,
            date: now,
        };

        let historical_answer = HistoricalAnswer::try_from(historical_answer_dto).unwrap();

        assert_eq!(historical_answer.id, "id");
        assert_eq!(historical_answer.user_id, "user_id");
        assert_eq!(historical_answer.task_id, "task_id");
        assert_eq!(historical_answer.date, now);
        assert!(historical_answer.was_valid);
    }
}
