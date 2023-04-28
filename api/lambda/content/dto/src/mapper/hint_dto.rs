use crate::{HintDto, MapFrom};
use content_domain::model::hint::Hint;

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
            content: "content".to_string(),
        };
        let hint_dto = HintDto::map_from(hint);
        assert_eq!(hint_dto.content, "content");
    }

    #[test]
    fn from_hint_dto() {
        let hint_dto = HintDto {
            content: "content".to_string(),
        };
        let hint = Hint::map_from(hint_dto);
        assert_eq!(hint.content, "content");
    }
}
