use content_domain::model::keyword::{Keyword, KeywordModifier};
use serde::{Deserialize, Serialize};

use crate::{MapFrom, MapInto};

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

impl MapFrom<KeywordModifier> for KeywordModifierDto {
    fn map_from(model: KeywordModifier) -> Self {
        match model {
            KeywordModifier::NewLine => Self::NewLine,
            KeywordModifier::AddIndentation => Self::AddIndentation,
            KeywordModifier::RemoveIndentation => Self::RemoveIndentation,
        }
    }
}

impl MapFrom<KeywordModifierDto> for KeywordModifier {
    fn map_from(dto: KeywordModifierDto) -> Self {
        match dto {
            KeywordModifierDto::NewLine => Self::NewLine,
            KeywordModifierDto::AddIndentation => Self::AddIndentation,
            KeywordModifierDto::RemoveIndentation => Self::RemoveIndentation,
        }
    }
}

impl MapFrom<Keyword> for KeywordDto {
    fn map_from(model: Keyword) -> Self {
        Self {
            id: model.id,
            content: model.content,
            modifiers: model.modifiers.map_into(),
        }
    }
}

impl MapFrom<KeywordDto> for Keyword {
    fn map_from(dto: KeywordDto) -> Self {
        Self {
            id: dto.id,
            content: dto.content,
            modifiers: dto.modifiers.map_into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_keyword_modifier() {
        assert_eq!(
            KeywordModifierDto::map_from(KeywordModifier::NewLine),
            KeywordModifierDto::NewLine
        );
        assert_eq!(
            KeywordModifierDto::map_from(KeywordModifier::AddIndentation),
            KeywordModifierDto::AddIndentation
        );
        assert_eq!(
            KeywordModifierDto::map_from(KeywordModifier::RemoveIndentation),
            KeywordModifierDto::RemoveIndentation
        );
    }

    #[test]
    fn from_keyword_modifier_dto() {
        assert_eq!(
            KeywordModifier::map_from(KeywordModifierDto::NewLine),
            KeywordModifier::NewLine
        );
        assert_eq!(
            KeywordModifier::map_from(KeywordModifierDto::AddIndentation),
            KeywordModifier::AddIndentation
        );
        assert_eq!(
            KeywordModifier::map_from(KeywordModifierDto::RemoveIndentation),
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
        let keyword_dto = KeywordDto::map_from(keyword);

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
        let keyword = Keyword::map_from(keyword_dto);

        assert_eq!(keyword.id, 1);
        assert_eq!(keyword.content, "keyword");
        assert_eq!(keyword.modifiers, vec![KeywordModifier::NewLine]);
    }
}
