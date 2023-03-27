use std::collections::HashMap;

use common_domain::identifiable::Identifiable;

use crate::into_modification::IntoModification;

use super::{
    historical_answer::HistoricalAnswer, modification::Modification,
    personalized_task::PersonalizedTask, task_content::TaskContent,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Task {
    pub id: String,
    pub section_id: String,
    pub position: Option<u64>,
    pub title: String,
    pub difficulty: u8,
    pub for_anonymous: bool,
    pub content: TaskContent,
}

impl Task {
    pub fn personalize(
        self,
        correct_historical_answers: &mut HashMap<String, HistoricalAnswer>,
    ) -> PersonalizedTask {
        let done_at = correct_historical_answers
            .remove(&self.id)
            .map(|answer| answer.created_at);

        PersonalizedTask {
            id: self.id,
            section_id: self.section_id,
            position: self.position,
            title: self.title,
            difficulty: self.difficulty,
            for_anonymous: self.for_anonymous,
            done_at,
            content: self.content,
        }
    }
}

impl IntoModification for Task {
    fn into_add_modification(self) -> Modification {
        Modification::AddTask(self)
    }

    fn into_remove_modification(self) -> Modification {
        Modification::RemoveTask(self)
    }

    fn into_update_modification(self) -> Modification {
        Modification::UpdateTask(self)
    }
}

impl Identifiable for Task {
    type Id = String;

    fn id(&self) -> &String {
        &self.id
    }
}

pub trait VecTaskExt {
    fn personalize(
        self,
        correct_historical_answers: HashMap<String, HistoricalAnswer>,
    ) -> Vec<PersonalizedTask>;
}

impl VecTaskExt for Vec<Task> {
    fn personalize(
        self,
        mut correct_historical_answers: HashMap<String, HistoricalAnswer>,
    ) -> Vec<PersonalizedTask> {
        self.into_iter()
            .map(|task| task.personalize(&mut correct_historical_answers))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_personalize() {
        let now = chrono::Utc::now();
        let mut correct_historical_answers = HashMap::new();
        correct_historical_answers.insert(
            "id".to_owned(),
            HistoricalAnswer {
                task_id: "id".to_owned(),
                created_at: now,
                ..Default::default()
            },
        );

        let tasks = vec![
            Task {
                id: "id".to_owned(),
                ..Default::default()
            },
            Task {
                id: "id_2".to_owned(),
                ..Default::default()
            },
        ];

        let personalized_tasks = tasks.personalize(correct_historical_answers);

        assert_eq!(
            personalized_tasks,
            vec![
                PersonalizedTask {
                    id: "id".to_owned(),
                    done_at: Some(now),
                    ..Default::default()
                },
                PersonalizedTask {
                    id: "id_2".to_owned(),
                    done_at: None,
                    ..Default::default()
                },
            ]
        );
    }

    #[test]
    fn personalized_answered_before() {
        let mut correct_historical_answers = HashMap::new();
        correct_historical_answers.insert(
            "id".to_owned(),
            HistoricalAnswer {
                task_id: "id".to_owned(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            },
        );

        let task = Task {
            id: "id".to_owned(),
            ..Default::default()
        };

        let personalized_task = task.personalize(&mut correct_historical_answers);

        assert!(personalized_task.done_at.is_some());
        assert!(correct_historical_answers.is_empty());
    }

    #[test]
    fn personalized_answered_after() {
        let mut correct_historical_answers = HashMap::new();
        correct_historical_answers.insert(
            "id".to_owned(),
            HistoricalAnswer {
                task_id: "id".to_owned(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            },
        );

        let task = Task {
            id: "id2".to_owned(),
            ..Default::default()
        };

        let personalized_task = task.personalize(&mut correct_historical_answers);

        assert!(personalized_task.done_at.is_none());
        assert!(!correct_historical_answers.is_empty());
    }

    #[test]
    fn test_into_modification() {
        let task = Task {
            id: "id".to_owned(),
            section_id: "section_id".to_owned(),
            title: "title".to_owned(),
            position: Some(0),
            difficulty: 1,
            for_anonymous: false,
            content: TaskContent::Empty,
        };

        assert_eq!(
            task.clone().into_add_modification(),
            Modification::AddTask(task.clone())
        );
        assert_eq!(
            task.clone().into_remove_modification(),
            Modification::RemoveTask(task.clone())
        );
        assert_eq!(
            task.clone().into_update_modification(),
            Modification::UpdateTask(task)
        );
    }

    #[test]
    fn test_id() {
        let task = Task {
            id: "id".to_owned(),
            section_id: "section_id".to_owned(),
            title: "title".to_owned(),
            difficulty: 1,
            position: Some(0),
            for_anonymous: false,
            content: TaskContent::Empty,
        };

        assert_eq!(task.id(), &"id".to_owned());
    }
}
