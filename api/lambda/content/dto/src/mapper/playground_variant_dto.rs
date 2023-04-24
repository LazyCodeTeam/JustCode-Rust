use crate::{FromDto, FromModel, PlaygroundVariationDto};
use content_domain::model::playground_variation::PlaygroundVariation;

impl FromModel<PlaygroundVariation> for PlaygroundVariationDto {
    fn from_model(model: PlaygroundVariation) -> Self {
        Self {
            content: model.content,
            description: model.description,
        }
    }
}

impl FromDto<PlaygroundVariationDto> for PlaygroundVariation {
    fn from_dto(dto: PlaygroundVariationDto) -> Self {
        Self {
            content: dto.content,
            description: dto.description,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_playground_variation() {
        let playground_variation = PlaygroundVariation {
            content: "content".to_string(),
            description: "description".to_string(),
        };
        let playground_variation_dto = PlaygroundVariationDto::from_model(playground_variation);
        assert_eq!(playground_variation_dto.content, "content");
        assert_eq!(playground_variation_dto.description, "description");
    }

    #[test]
    fn from_playground_variation_dto() {
        let playground_variation_dto = PlaygroundVariationDto {
            content: "content".to_string(),
            description: "description".to_string(),
        };
        let playground_variation = PlaygroundVariation::from_dto(playground_variation_dto);
        assert_eq!(playground_variation.content, "content");
        assert_eq!(playground_variation.description, "description");
    }
}
