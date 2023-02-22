use serde::{Deserialize, Serialize};

use super::section_preview_dto::SectionPreviewDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TechnologyDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections_preview: Vec<SectionPreviewDto>,
}
