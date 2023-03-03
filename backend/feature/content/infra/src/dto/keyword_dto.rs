use content_domain::model::keyword::{Keyword, KeywordModifier};
use serde::{Deserialize, Serialize};

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

impl From<KeywordModifier> for KeywordModifierDto {
    fn from(modifier: KeywordModifier) -> Self {
        match modifier {
            KeywordModifier::NewLine => Self::NewLine,
            KeywordModifier::AddIndentation => Self::AddIndentation,
            KeywordModifier::RemoveIndentation => Self::RemoveIndentation,
        }
    }
}

impl From<KeywordModifierDto> for KeywordModifier {
    fn from(modifier_dto: KeywordModifierDto) -> Self {
        match modifier_dto {
            KeywordModifierDto::NewLine => Self::NewLine,
            KeywordModifierDto::AddIndentation => Self::AddIndentation,
            KeywordModifierDto::RemoveIndentation => Self::RemoveIndentation,
        }
    }
}

impl From<Keyword> for KeywordDto {
    fn from(keyword: Keyword) -> Self {
        Self {
            id: keyword.id,
            content: keyword.content,
            modifiers: keyword.modifiers.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<KeywordDto> for Keyword {
    fn from(keyword_dto: KeywordDto) -> Self {
        Self {
            id: keyword_dto.id,
            content: keyword_dto.content,
            modifiers: keyword_dto.modifiers.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_keyword_modifier() {
        assert_eq!(
            KeywordModifierDto::from(KeywordModifier::NewLine),
            KeywordModifierDto::NewLine
        );
        assert_eq!(
            KeywordModifierDto::from(KeywordModifier::AddIndentation),
            KeywordModifierDto::AddIndentation
        );
        assert_eq!(
            KeywordModifierDto::from(KeywordModifier::RemoveIndentation),
            KeywordModifierDto::RemoveIndentation
        );
    }

    #[test]
    fn from_keyword_modifier_dto() {
        assert_eq!(
            KeywordModifier::from(KeywordModifierDto::NewLine),
            KeywordModifier::NewLine
        );
        assert_eq!(
            KeywordModifier::from(KeywordModifierDto::AddIndentation),
            KeywordModifier::AddIndentation
        );
        assert_eq!(
            KeywordModifier::from(KeywordModifierDto::RemoveIndentation),
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
        let keyword_dto = KeywordDto::from(keyword);

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
        let keyword = Keyword::from(keyword_dto);

        assert_eq!(keyword.id, 1);
        assert_eq!(keyword.content, "keyword");
        assert_eq!(keyword.modifiers, vec![KeywordModifier::NewLine]);
    }
}
