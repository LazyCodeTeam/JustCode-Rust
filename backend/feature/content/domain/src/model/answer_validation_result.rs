#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnswerValidationResult {
    pub is_valid: bool,
    pub had_valid_answer_before: bool,
}
