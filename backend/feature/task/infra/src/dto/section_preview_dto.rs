use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SectionPreviewDto {
    pub id: i32,
    pub name: String,
}
