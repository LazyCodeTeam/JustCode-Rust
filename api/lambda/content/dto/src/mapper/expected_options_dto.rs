use content_domain::model::option_data::OptionData;

use crate::{ExpectedOptionDto, FromDto};

impl FromDto<Vec<ExpectedOptionDto>> for Vec<OptionData> {
    fn from_dto(dto: Vec<ExpectedOptionDto>) -> Self {
        dto.into_iter()
            .enumerate()
            .map(|(i, option)| OptionData {
                id: i as u16,
                content: option.content,
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::IntoModel;

    use super::*;

    #[test]
    fn test_from() {
        let options = vec![
            ExpectedOptionDto {
                content: "Option 1".to_string(),
            },
            ExpectedOptionDto {
                content: "Option 2".to_string(),
            },
        ];

        let options_data: Vec<OptionData> = options.into_model();

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
