use content_domain::model::keyword::KeywordModifier;
use serde::{Deserialize, Serialize};

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
    fn from(modifier: KeywordModifierDto) -> Self {
        match modifier {
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
}
