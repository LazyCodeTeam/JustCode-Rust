use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct PlaygroundVariationDto {
    pub id: String,
    pub content: String,
    pub description: String,
}
