use crate::{FromModel, OptionDto};
use content_domain::model::option_data::OptionData;

impl FromModel<OptionData> for OptionDto {
    fn from_model(model: OptionData) -> Self {
        Self {
            id: model.id,
            content: model.content,
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
        let option_data_dto = OptionDto::from_model(option_data);
        assert_eq!(option_data_dto.id, 1);
        assert_eq!(option_data_dto.content, "content");
    }
}
