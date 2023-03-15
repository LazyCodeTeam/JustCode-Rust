use crate::{FromDto, FromModel, HintDto};
use content_domain::model::hint::Hint;

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
            content: "content".to_string(),
        };
        let hint_dto = HintDto::from_model(hint);
        assert_eq!(hint_dto.content, "content");
    }

    #[test]
    fn from_hint_dto() {
        let hint_dto = HintDto {
            content: "content".to_string(),
        };
        let hint = Hint::from_dto(hint_dto);
        assert_eq!(hint.content, "content");
    }
}
