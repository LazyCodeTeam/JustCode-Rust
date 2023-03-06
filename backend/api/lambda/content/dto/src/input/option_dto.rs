use content_domain::model::option_data::OptionData;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
pub struct OptionDto {
    pub content: String,
}

pub(crate) struct OptionsData(Vec<OptionData>);

impl From<OptionsData> for Vec<OptionData> {
    fn from(value: OptionsData) -> Self {
        value.0
    }
}

impl From<Vec<OptionDto>> for OptionsData {
    fn from(value: Vec<OptionDto>) -> Self {
        OptionsData(
            value
                .into_iter()
                .enumerate()
                .map(|(i, option)| OptionData {
                    id: i.try_into().unwrap(),
                    content: option.content,
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let options = vec![
            OptionDto {
                content: "Option 1".to_string(),
            },
            OptionDto {
                content: "Option 2".to_string(),
            },
        ];

        let options_data: Vec<OptionData> = OptionsData::from(options).into();

        assert_eq!(
            options_data,
            vec![
                OptionData {
                    id: 0,
                    content: "Option 1".to_string(),
                },
                OptionData {
                    id: 1,
                    content: "Option 2".to_string(),
                },
            ]
        );
    }
}
