use std::collections::HashMap;

use crate::Personalize;

use super::{
    historical_answer::HistoricalAnswer, personalized_task_preview::PersonalizedTaskPreview,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TaskPreview {
    pub id: String,
    pub title: String,
    pub for_anonymous: bool,
}

impl Personalize<PersonalizedTaskPreview> for TaskPreview {
    fn personalize(
        self,
        correct_historical_answers: &HashMap<String, HistoricalAnswer>,
    ) -> PersonalizedTaskPreview {
        let done_at = correct_historical_answers
            .get(&self.id)
            .map(|historical_answer| historical_answer.created_at);

        PersonalizedTaskPreview {
            id: self.id,
            title: self.title,
            for_anonymous: self.for_anonymous,
            done_at,
        }
    }
}
