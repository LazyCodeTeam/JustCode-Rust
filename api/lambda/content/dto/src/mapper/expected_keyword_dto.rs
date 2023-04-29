use crate::{ExpectedKeywordDto, MapFrom, MapInto};
use content_domain::model::keyword::Keyword;

impl MapFrom<Vec<ExpectedKeywordDto>> for Vec<Keyword> {
    fn map_from(dto: Vec<ExpectedKeywordDto>) -> Self {
        dto.into_iter()
            .enumerate()
            .map(|(i, keyword)| Keyword {
                id: i.try_into().unwrap(),
                content: keyword.content,
                modifiers: keyword
                    .modifiers
                    .into_iter()
                    .map(MapInto::map_into)
                    .collect(),
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::KeywordModifierDto;
    use content_domain::model::keyword::KeywordModifier;

    use super::*;

    #[test]
    fn map_keywords_dtos() {
        let dtos = vec![
            ExpectedKeywordDto {
                content: "content".to_string(),
                modifiers: vec![KeywordModifierDto::NewLine],
            },
            ExpectedKeywordDto {
                content: "content2".to_string(),
                modifiers: vec![KeywordModifierDto::AddIndentation],
            },
        ];

        let keywords = Vec::<Keyword>::map_from(dtos);

        assert_eq!(
            Into::<Vec<Keyword>>::into(keywords),
            vec![
                Keyword {
                    id: 0,
                    content: "content".to_string(),
                    modifiers: vec![KeywordModifier::NewLine],
                },
                Keyword {
                    id: 1,
                    content: "content2".to_string(),
                    modifiers: vec![KeywordModifier::AddIndentation],
                },
            ]
        );
    }
}
