use crate::{KeywordDto, MapFrom, MapInto};
use content_domain::model::keyword::Keyword;

impl MapFrom<Keyword> for KeywordDto {
    fn map_from(model: Keyword) -> Self {
        Self {
            id: model.id,
            content: model.content,
            modifiers: model.modifiers.into_iter().map(MapInto::map_into).collect(),
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
        let keyword_dto = KeywordDto::map_from(keyword);
        assert_eq!(keyword_dto.id, 1);
        assert_eq!(keyword_dto.content, "content");
        assert_eq!(keyword_dto.modifiers, vec![KeywordModifierDto::NewLine]);
    }
}
