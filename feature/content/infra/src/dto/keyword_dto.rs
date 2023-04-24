use content_domain::model::keyword::{Keyword, KeywordModifier};
use serde::{Deserialize, Serialize};

use crate::{FromDto, FromModel, IntoDto, IntoModel};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct KeywordDto {
    pub id: u16,
    pub content: String,
    pub modifiers: Vec<KeywordModifierDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeywordModifierDto {
    NewLine,
    AddIndentation,
    RemoveIndentation,
}

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

impl FromModel<Keyword> for KeywordDto {
    fn from_model(model: Keyword) -> Self {
        Self {
            id: model.id,
            content: model.content,
            modifiers: model.modifiers.into_dto(),
        }
    }
}

impl FromDto<KeywordDto> for Keyword {
    fn from_dto(dto: KeywordDto) -> Self {
        Self {
            id: dto.id,
            content: dto.content,
            modifiers: dto.modifiers.into_model(),
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

    #[test]
    fn from_keyword() {
        let keyword = Keyword {
            id: 1,
            content: "keyword".to_string(),
            modifiers: vec![KeywordModifier::NewLine],
        };
        let keyword_dto = KeywordDto::from_model(keyword);

        assert_eq!(keyword_dto.id, 1);
        assert_eq!(keyword_dto.content, "keyword");
        assert_eq!(keyword_dto.modifiers, vec![KeywordModifierDto::NewLine]);
    }

    #[test]
    fn from_keyword_dto() {
        let keyword_dto = KeywordDto {
            id: 1,
            content: "keyword".to_string(),
            modifiers: vec![KeywordModifierDto::NewLine],
        };
        let keyword = Keyword::from_dto(keyword_dto);

        assert_eq!(keyword.id, 1);
        assert_eq!(keyword.content, "keyword");
        assert_eq!(keyword.modifiers, vec![KeywordModifier::NewLine]);
    }
}
