use super::answer::Answer;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnswerToSave {
    pub user_id: String,
    pub had_valid_answer_before: bool,
    pub is_valid: bool,
    pub answer: Answer,
}
