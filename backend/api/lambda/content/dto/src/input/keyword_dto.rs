use crate::common::keyword_modifier_dto::KeywordModifierDto;
use content_domain::model::keyword::Keyword;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
pub struct KeywordDto {
    pub content: String,
    pub modifiers: Vec<KeywordModifierDto>,
}

#[derive(Debug)]
pub(crate) struct Keywords(Vec<Keyword>);

impl From<Keywords> for Vec<Keyword> {
    fn from(value: Keywords) -> Self {
        value.0
    }
}

impl From<Vec<KeywordDto>> for Keywords {
    fn from(value: Vec<KeywordDto>) -> Self {
        Keywords(
            value
                .into_iter()
                .enumerate()
                .map(|(i, keyword)| Keyword {
                    id: i.try_into().unwrap(),
                    content: keyword.content,
                    modifiers: keyword.modifiers.into_iter().map(Into::into).collect(),
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use content_domain::model::keyword::KeywordModifier;

    use super::*;

    #[test]
    fn map_keywords_dtos() {
        let dtos = vec![
            KeywordDto {
                content: "content".to_string(),
                modifiers: vec![KeywordModifierDto::NewLine],
            },
            KeywordDto {
                content: "content2".to_string(),
                modifiers: vec![KeywordModifierDto::AddIndentation],
            },
        ];

        let keywords = Keywords::from(dtos);

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
