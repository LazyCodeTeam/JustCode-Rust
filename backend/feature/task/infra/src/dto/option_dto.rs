use serde::{Deserialize, Serialize};
use task_domain::model::option_data::OptionData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OptionDataDto {
    pub id: u16,
    pub content: String,
}

impl From<OptionData> for OptionDataDto {
    fn from(option_data: OptionData) -> Self {
        Self {
            id: option_data.id,
            content: option_data.content,
        }
    }
}

impl From<OptionDataDto> for OptionData {
    fn from(option_data_dto: OptionDataDto) -> Self {
        Self {
            id: option_data_dto.id,
            content: option_data_dto.content,
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
        let option_data_dto = OptionDataDto::from(option_data);
        assert_eq!(option_data_dto.id, 1);
        assert_eq!(option_data_dto.content, "option");
    }

    #[test]
    fn from_option_data_dto() {
        let option_data_dto = OptionDataDto {
            id: 1,
            content: "option".to_string(),
        };
        let option_data = OptionData::from(option_data_dto);
        assert_eq!(option_data.id, 1);
        assert_eq!(option_data.content, "option");
    }
}
