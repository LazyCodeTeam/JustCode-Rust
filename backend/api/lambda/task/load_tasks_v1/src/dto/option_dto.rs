use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct OptionDto {
    pub id: String,
    pub content: String,
}
