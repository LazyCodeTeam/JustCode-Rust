use content_domain::model::option_data::OptionData;
use serde::{Deserialize, Serialize};

use crate::MapFrom;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OptionDataDto {
    pub id: u16,
    pub content: String,
}

impl MapFrom<OptionData> for OptionDataDto {
    fn map_from(model: OptionData) -> Self {
        Self {
            id: model.id,
            content: model.content,
        }
    }
}

impl MapFrom<OptionDataDto> for OptionData {
    fn map_from(dto: OptionDataDto) -> Self {
        Self {
            id: dto.id,
            content: dto.content,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn from_option_data() {
        let option_data = OptionData {
            id: 1,
            content: "option".to_string(),
        };
        let option_data_dto = OptionDataDto::map_from(option_data);
        assert_eq!(option_data_dto.id, 1);
        assert_eq!(option_data_dto.content, "option");
    }

    #[test]
    fn from_option_data_dto() {
        let option_data_dto = OptionDataDto {
            id: 1,
            content: "option".to_string(),
        };
        let option_data = OptionData::map_from(option_data_dto);
        assert_eq!(option_data.id, 1);
        assert_eq!(option_data.content, "option");
    }
}
