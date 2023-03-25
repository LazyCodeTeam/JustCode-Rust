use super::{answer::Answer, answer_result::AnswerResult};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnswerToSave {
    pub user_id: String,
    pub result: AnswerResult,
    pub answer: Answer,
}
