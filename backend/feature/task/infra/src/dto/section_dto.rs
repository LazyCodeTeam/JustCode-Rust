use serde::{Deserialize, Serialize};

use super::task_preview_dto::TaskPreviewDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SectionDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub technology_id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks_preview: Vec<TaskPreviewDto>,
}
