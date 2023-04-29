use content_domain::model::hint::Hint;
use serde::{Deserialize, Serialize};

use crate::MapFrom;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct HintDto {
    pub content: String,
}

impl MapFrom<Hint> for HintDto {
    fn map_from(model: Hint) -> Self {
        Self {
            content: model.content,
        }
    }
}

impl MapFrom<HintDto> for Hint {
    fn map_from(dto: HintDto) -> Self {
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
        let hint_dto = HintDto::map_from(hint);
        assert_eq!(hint_dto.content, "hint");
    }

    #[test]
    fn from_hint_dto() {
        let hint_dto = HintDto {
            content: "hint".to_string(),
        };
        let hint = Hint::map_from(hint_dto);
        assert_eq!(hint.content, "hint");
    }
}
