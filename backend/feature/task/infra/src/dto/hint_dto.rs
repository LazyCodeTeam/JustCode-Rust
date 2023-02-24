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

impl From<HintDto> for Hint {
    fn from(hint_dto: HintDto) -> Self {
        Self {
            content: hint_dto.content,
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

    #[test]
    fn from_hint_dto() {
        let hint_dto = HintDto {
            content: "hint".to_string(),
        };
        let hint = Hint::from(hint_dto);
        assert_eq!(hint.content, "hint");
    }
}
