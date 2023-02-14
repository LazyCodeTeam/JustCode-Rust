use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct OptionDto {
    pub content: String,
}
