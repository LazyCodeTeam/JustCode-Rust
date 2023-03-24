use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HistoricalAnswer {
    pub id: String,
    pub user_id: String,
    pub task_id: String,
    pub was_valid: bool,
    pub date: DateTime<Utc>,
}

pub trait VecHistoricalAnswerExt {
    fn had_valid_answer(&self) -> bool;
}

impl VecHistoricalAnswerExt for Vec<HistoricalAnswer> {
    fn had_valid_answer(&self) -> bool {
        self.iter().any(|answer| answer.was_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn had_valid_answer() {
        let answers = vec![
            HistoricalAnswer {
                id: "id".to_string(),
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                was_valid: false,
                date: Utc::now(),
            },
            HistoricalAnswer {
                id: "id".to_string(),
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                was_valid: true,
                date: Utc::now(),
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
                id: "id".to_string(),
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                was_valid: false,
                date: Utc::now(),
            },
            HistoricalAnswer {
                id: "id".to_string(),
                user_id: "user_id".to_string(),
                task_id: "task_id".to_string(),
                was_valid: false,
                date: Utc::now(),
            },
        ];

        assert!(!answers.had_valid_answer());
    }
}
