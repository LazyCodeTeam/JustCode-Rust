use crate::{ExpectedKeywordDto, FromDto, IntoModel};
use content_domain::model::keyword::Keyword;

impl FromDto<Vec<ExpectedKeywordDto>> for Vec<Keyword> {
    fn from_dto(dto: Vec<ExpectedKeywordDto>) -> Self {
        dto.into_iter()
            .enumerate()
            .map(|(i, keyword)| Keyword {
                id: i.try_into().unwrap(),
                content: keyword.content,
                modifiers: keyword
                    .modifiers
                    .into_iter()
                    .map(IntoModel::into_model)
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

        let keywords = Vec::<Keyword>::from_dto(dtos);

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
