use crate::{FromDto, FromModel, KeywordModifierDto};
use content_domain::model::keyword::KeywordModifier;

impl FromModel<KeywordModifier> for KeywordModifierDto {
    fn from_model(model: KeywordModifier) -> Self {
        match model {
            KeywordModifier::NewLine => Self::NewLine,
            KeywordModifier::AddIndentation => Self::AddIndentation,
            KeywordModifier::RemoveIndentation => Self::RemoveIndentation,
        }
    }
}

impl FromDto<KeywordModifierDto> for KeywordModifier {
    fn from_dto(dto: KeywordModifierDto) -> Self {
        match dto {
            KeywordModifierDto::NewLine => Self::NewLine,
            KeywordModifierDto::AddIndentation => Self::AddIndentation,
            KeywordModifierDto::RemoveIndentation => Self::RemoveIndentation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_keyword_modifier() {
        assert_eq!(
            KeywordModifierDto::from_model(KeywordModifier::NewLine),
            KeywordModifierDto::NewLine
        );
        assert_eq!(
            KeywordModifierDto::from_model(KeywordModifier::AddIndentation),
            KeywordModifierDto::AddIndentation
        );
        assert_eq!(
            KeywordModifierDto::from_model(KeywordModifier::RemoveIndentation),
            KeywordModifierDto::RemoveIndentation
        );
    }

    #[test]
    fn from_keyword_modifier_dto() {
        assert_eq!(
            KeywordModifier::from_dto(KeywordModifierDto::NewLine),
            KeywordModifier::NewLine
        );
        assert_eq!(
            KeywordModifier::from_dto(KeywordModifierDto::AddIndentation),
            KeywordModifier::AddIndentation
        );
        assert_eq!(
            KeywordModifier::from_dto(KeywordModifierDto::RemoveIndentation),
            KeywordModifier::RemoveIndentation
        );
    }
}
