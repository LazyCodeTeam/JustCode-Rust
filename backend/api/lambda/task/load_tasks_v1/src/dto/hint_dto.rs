use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct HintDto {
    pub content: String,
}
