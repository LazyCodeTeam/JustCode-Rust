use std::collections::{HashMap, HashSet};

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

    fn into_tasks_ids(self) -> HashSet<String>;

    fn into_answer_per_task_id(self) -> HashMap<String, HistoricalAnswer>;
}

impl VecHistoricalAnswerExt for Vec<HistoricalAnswer> {
    fn had_valid_answer(&self) -> bool {
        self.iter().any(|answer| answer.result.is_valid())
    }

    fn into_tasks_ids(self) -> HashSet<String> {
        self.into_iter().map(|answer| answer.task_id).collect()
    }

    fn into_answer_per_task_id(self) -> HashMap<String, HistoricalAnswer> {
        self.into_iter()
            .map(|answer| (answer.task_id.clone(), answer))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into_answer_per_task_id() {
        let answers = vec![
            HistoricalAnswer {
                task_id: "task_id".to_string(),
                ..Default::default()
            },
            HistoricalAnswer {
                task_id: "task_id_2".to_string(),
                ..Default::default()
            },
        ];

        let answers_per_task_id = answers.into_answer_per_task_id();

        assert_eq!(answers_per_task_id.len(), 2);
        assert_eq!(
            answers_per_task_id.get("task_id"),
            Some(&HistoricalAnswer {
                task_id: "task_id".to_string(),
                ..Default::default()
            })
        );
        assert_eq!(
            answers_per_task_id.get("task_id_2"),
            Some(&HistoricalAnswer {
                task_id: "task_id_2".to_string(),
                ..Default::default()
            })
        );
    }

    #[test]
    fn into_ids() {
        let answers = vec![
            HistoricalAnswer {
                task_id: "task_id".to_string(),
                ..Default::default()
            },
            HistoricalAnswer {
                task_id: "task_id_2".to_string(),
                ..Default::default()
            },
        ];

        let ids = answers.into_tasks_ids();

        assert_eq!(ids.len(), 2);
        assert!(ids.contains("task_id"));
        assert!(ids.contains("task_id_2"));
    }

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
