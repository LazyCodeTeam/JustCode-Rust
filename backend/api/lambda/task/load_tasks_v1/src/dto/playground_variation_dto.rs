use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct PlaygroundVariationDto {
    pub content: String,
    pub description: String,
}
