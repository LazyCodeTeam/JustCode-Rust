use serde::Deserialize;
use task_domain::model::playground_variation::PlaygroundVariation;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
pub struct PlaygroundVariationDto {
    pub content: String,
    pub description: String,
}

impl From<PlaygroundVariationDto> for PlaygroundVariation {
    fn from(value: PlaygroundVariationDto) -> Self {
        PlaygroundVariation {
            content: value.content,
            description: value.description,
        }
    }
}
