use serde::{Deserialize, Serialize};
use task_domain::model::hint::Hint;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct HintDto {
    pub content: String,
}

impl From<Hint> for HintDto {
    fn from(hint: Hint) -> Self {
        Self {
            content: hint.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hint() {
        let hint = Hint {
            content: "hint".to_string(),
        };
        let hint_dto = HintDto::from(hint);
        assert_eq!(hint_dto.content, "hint");
    }
}
