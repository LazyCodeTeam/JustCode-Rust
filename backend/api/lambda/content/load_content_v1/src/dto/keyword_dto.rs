use content_domain::model::keyword::{Keyword, KeywordModifier};
#[cfg(feature = "fake_dto")]
use fake::{Dummy, Fake};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
#[cfg_attr(feature = "fake_dto", derive(Dummy, serde::Serialize))]
pub struct KeywordDto {
    pub content: String,
    pub modifiers: Vec<ModifierDto>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[cfg_attr(feature = "fake", derive(fake::Dummy, serde::Serialize))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ModifierDto {
    NewLine,
    AddIndentation,
    RemoveIndentation,
}

impl From<ModifierDto> for KeywordModifier {
    fn from(value: ModifierDto) -> Self {
        match value {
            ModifierDto::NewLine => KeywordModifier::NewLine,
            ModifierDto::AddIndentation => KeywordModifier::AddIndentation,
            ModifierDto::RemoveIndentation => KeywordModifier::RemoveIndentation,
        }
    }
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
                    modifiers: keyword
                        .modifiers
                        .into_iter()
                        .map(KeywordModifier::from)
                        .collect(),
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_modifier_dto() {
        assert_eq!(
            KeywordModifier::from(ModifierDto::NewLine),
            KeywordModifier::NewLine
        );
        assert_eq!(
            KeywordModifier::from(ModifierDto::AddIndentation),
            KeywordModifier::AddIndentation
        );
        assert_eq!(
            KeywordModifier::from(ModifierDto::RemoveIndentation),
            KeywordModifier::RemoveIndentation
        );
    }

    #[test]
    fn map_keywords_dtos() {
        let dtos = vec![
            KeywordDto {
                content: "content".to_string(),
                modifiers: vec![ModifierDto::NewLine],
            },
            KeywordDto {
                content: "content2".to_string(),
                modifiers: vec![ModifierDto::AddIndentation],
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
