use content_domain::model::option_data::OptionData;

use crate::{ExpectedOptionDto, MapFrom};

impl MapFrom<Vec<ExpectedOptionDto>> for Vec<OptionData> {
    fn map_from(dto: Vec<ExpectedOptionDto>) -> Self {
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
    use crate::MapInto;

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

        let options_data: Vec<OptionData> = options.map_into();

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
