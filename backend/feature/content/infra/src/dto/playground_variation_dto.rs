use content_domain::model::playground_variation::PlaygroundVariation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlaygroundVariationDto {
    pub content: String,
    pub description: String,
}

impl From<PlaygroundVariation> for PlaygroundVariationDto {
    fn from(playground_variation: PlaygroundVariation) -> Self {
        Self {
            content: playground_variation.content,
            description: playground_variation.description,
        }
    }
}

impl From<PlaygroundVariationDto> for PlaygroundVariation {
    fn from(playground_variation_dto: PlaygroundVariationDto) -> Self {
        Self {
            content: playground_variation_dto.content,
            description: playground_variation_dto.description,
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
        let playground_variation_dto = PlaygroundVariationDto::from(playground_variation);
        assert_eq!(playground_variation_dto.content, "content");
        assert_eq!(playground_variation_dto.description, "description");
    }

    #[test]
    fn from_playground_variation_dto() {
        let playground_variation_dto = PlaygroundVariationDto {
            content: "content".to_string(),
            description: "description".to_string(),
        };
        let playground_variation = PlaygroundVariation::from(playground_variation_dto);
        assert_eq!(playground_variation.content, "content");
        assert_eq!(playground_variation.description, "description");
    }
}