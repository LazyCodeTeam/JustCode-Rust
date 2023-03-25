use chrono::{DateTime, Utc};

use super::answer_result::AnswerResult;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HistoricalAnswer {
    pub user_id: String,
    pub task_id: String,
    pub result: AnswerResult,
    pub created_at: DateTime<Utc>,
}

pub trait VecHistoricalAnswerExt {
    fn had_valid_answer(&self) -> bool;
}

impl VecHistoricalAnswerExt for Vec<HistoricalAnswer> {
    fn had_valid_answer(&self) -> bool {
        self.iter().any(|answer| answer.result.is_valid())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn had_valid_answer() {
        let answers = vec![
            HistoricalAnswer {
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                result: AnswerResult::Invalid,
                created_at: Utc::now(),
            },
            HistoricalAnswer {
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                result: AnswerResult::Valid,
                created_at: Utc::now(),
            },
        ];

        assert!(answers.had_valid_answer());
    }

    #[test]
    fn had_valid_answer_empty() {
        let answers = Vec::new();

        assert!(!answers.had_valid_answer());
    }

    #[test]
    fn no_valid_answer() {
        let answers = vec![
            HistoricalAnswer {
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                result: AnswerResult::Invalid,
                created_at: Utc::now(),
            },
            HistoricalAnswer {
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                result: AnswerResult::Invalid,
                created_at: Utc::now(),
            },
        ];

        assert!(!answers.had_valid_answer());
    }
}
