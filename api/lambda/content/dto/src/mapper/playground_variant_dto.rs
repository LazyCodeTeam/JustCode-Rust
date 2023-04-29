use crate::{MapFrom, PlaygroundVariationDto};
use content_domain::model::playground_variation::PlaygroundVariation;

impl MapFrom<PlaygroundVariation> for PlaygroundVariationDto {
    fn map_from(model: PlaygroundVariation) -> Self {
        Self {
            content: model.content,
            description: model.description,
        }
    }
}

impl MapFrom<PlaygroundVariationDto> for PlaygroundVariation {
    fn map_from(dto: PlaygroundVariationDto) -> Self {
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
        let playground_variation_dto = PlaygroundVariationDto::map_from(playground_variation);
        assert_eq!(playground_variation_dto.content, "content");
        assert_eq!(playground_variation_dto.description, "description");
    }

    #[test]
    fn from_playground_variation_dto() {
        let playground_variation_dto = PlaygroundVariationDto {
            content: "content".to_string(),
            description: "description".to_string(),
        };
        let playground_variation = PlaygroundVariation::map_from(playground_variation_dto);
        assert_eq!(playground_variation.content, "content");
        assert_eq!(playground_variation.description, "description");
    }
}
