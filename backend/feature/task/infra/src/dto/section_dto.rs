use serde::{Deserialize, Serialize};
use task_domain::model::section::Section;

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

impl From<Section> for SectionDto {
    fn from(section: Section) -> Self {
        Self {
            id: section.id,
            technology_id: section.technology_id,
            title: section.title,
            description: section.description,
            image: section.image,
            tasks_preview: section.tasks_preview.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use task_domain::model::task_preview::TaskPreview;

    use super::*;

    #[test]
    fn from_section() {
        let tasks_preview = vec![TaskPreview::default()];

        let section = Section {
            id: "id".to_string(),
            technology_id: "technology_id".to_string(),
            title: "title".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            tasks_preview: tasks_preview.clone(),
        };

        let section_dto = SectionDto::from(section);

        assert_eq!(
            section_dto,
            SectionDto {
                id: "id".to_string(),
                technology_id: "technology_id".to_string(),
                title: "title".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                tasks_preview: tasks_preview.into_iter().map(Into::into).collect(),
            }
        );
    }
}
