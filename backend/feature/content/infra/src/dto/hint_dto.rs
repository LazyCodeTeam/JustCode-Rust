use content_domain::model::hint::Hint;
use serde::{Deserialize, Serialize};

use crate::{FromDto, FromModel};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct HintDto {
    pub content: String,
}

impl FromModel<Hint> for HintDto {
    fn from_model(model: Hint) -> Self {
        Self {
            content: model.content,
        }
    }
}

impl FromDto<HintDto> for Hint {
    fn from_dto(dto: HintDto) -> Self {
        Self {
            content: dto.content,
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
        let hint_dto = HintDto::from_model(hint);
        assert_eq!(hint_dto.content, "hint");
    }

    #[test]
    fn from_hint_dto() {
        let hint_dto = HintDto {
            content: "hint".to_string(),
        };
        let hint = Hint::from_dto(hint_dto);
        assert_eq!(hint.content, "hint");
    }
}
