use content_domain::model::keyword::Keyword;
use serde::{Deserialize, Serialize};

use crate::common::keyword_modifier_dto::KeywordModifierDto;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeywordDto {
    pub id: u16,
    pub content: String,
    pub modifiers: Vec<KeywordModifierDto>,
}

impl From<Keyword> for KeywordDto {
    fn from(keyword: Keyword) -> Self {
        Self {
            id: keyword.id,
            content: keyword.content,
            modifiers: keyword
                .modifiers
                .into_iter()
                .map(KeywordModifierDto::from)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::keyword::KeywordModifier;

    use super::*;

    #[test]
    fn from_keyword() {
        let keyword = Keyword {
            id: 1,
            content: "content".to_string(),
            modifiers: vec![KeywordModifier::NewLine],
        };
        let keyword_dto = KeywordDto::from(keyword);
        assert_eq!(keyword_dto.id, 1);
        assert_eq!(keyword_dto.content, "content");
        assert_eq!(keyword_dto.modifiers, vec![KeywordModifierDto::NewLine]);
    }
}
