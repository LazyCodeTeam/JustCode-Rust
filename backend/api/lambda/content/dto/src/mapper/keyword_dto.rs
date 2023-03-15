use crate::{FromModel, IntoDto, KeywordDto};
use content_domain::model::keyword::Keyword;

impl FromModel<Keyword> for KeywordDto {
    fn from_model(model: Keyword) -> Self {
        Self {
            id: model.id,
            content: model.content,
            modifiers: model.modifiers.into_iter().map(IntoDto::into_dto).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::KeywordModifierDto;
    use content_domain::model::keyword::KeywordModifier;

    use super::*;

    #[test]
    fn from_keyword() {
        let keyword = Keyword {
            id: 1,
            content: "content".to_string(),
            modifiers: vec![KeywordModifier::NewLine],
        };
        let keyword_dto = KeywordDto::from_model(keyword);
        assert_eq!(keyword_dto.id, 1);
        assert_eq!(keyword_dto.content, "content");
        assert_eq!(keyword_dto.modifiers, vec![KeywordModifierDto::NewLine]);
    }
}
