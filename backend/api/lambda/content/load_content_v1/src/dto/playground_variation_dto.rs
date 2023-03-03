use content_domain::model::playground_variation::PlaygroundVariation;
#[cfg(feature = "fake_dto")]
use fake::{Dummy, Fake};
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
#[cfg_attr(feature = "fake_dto", derive(Dummy, serde::Serialize))]
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
