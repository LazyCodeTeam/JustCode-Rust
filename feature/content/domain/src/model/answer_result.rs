#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum AnswerResult {
    #[default]
    Invalid,
    Valid,
    AgainValid,
}

impl AnswerResult {
    pub fn new(is_valid: bool, had_valid_answer_before: bool) -> Self {
        match (is_valid, had_valid_answer_before) {
            (true, false) => Self::Valid,
            (true, true) => Self::AgainValid,
            _ => Self::Invalid,
        }
    }
    pub fn is_valid(&self) -> bool {
        !matches!(self, AnswerResult::Invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid() {
        assert!(!AnswerResult::Invalid.is_valid());
        assert!(AnswerResult::Valid.is_valid());
        assert!(AnswerResult::AgainValid.is_valid());
    }

    #[test]
    fn new() {
        assert_eq!(AnswerResult::new(false, false), AnswerResult::Invalid);
        assert_eq!(AnswerResult::new(true, false), AnswerResult::Valid);
        assert_eq!(AnswerResult::new(true, true), AnswerResult::AgainValid);
    }
}
