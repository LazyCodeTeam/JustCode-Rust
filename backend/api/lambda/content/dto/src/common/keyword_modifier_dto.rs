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
}
