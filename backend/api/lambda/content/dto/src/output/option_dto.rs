use content_domain::model::option_data::OptionData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OptionDto {
    pub id: u16,
    pub content: String,
}

impl From<OptionData> for OptionDto {
    fn from(option_data: OptionData) -> Self {
        Self {
            id: option_data.id,
            content: option_data.content,
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
            content: "content".to_string(),
        };
        let option_data_dto = OptionDto::from(option_data);
        assert_eq!(option_data_dto.id, 1);
        assert_eq!(option_data_dto.content, "content");
    }
}
