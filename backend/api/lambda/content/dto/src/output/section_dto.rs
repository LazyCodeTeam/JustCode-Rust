use content_domain::model::section::Section;
use serde::{Deserialize, Serialize};

use super::task_preview_dto::TaskPreviewDto;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SectionDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks_preview: Vec<TaskPreviewDto>,
}

impl From<Section> for SectionDto {
    fn from(section: Section) -> Self {
        Self {
            id: section.id,
            title: section.title,
            description: section.description,
            image: section.image,
            tasks_preview: section
                .tasks_preview
                .into_iter()
                .map(|task_preview| task_preview.into())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::task_preview::TaskPreview;

    use super::*;

    #[test]
    fn from_section() {
        let section = Section {
            id: "id".to_string(),
            position: 0,
            technology_id: "technology_id".to_string(),
            title: "title".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            tasks_preview: vec![TaskPreview {
                id: "id".to_string(),
                title: "title".to_string(),
                for_anonymous: true,
            }],
        };

        let section_dto = SectionDto::from(section);

        assert_eq!(
            section_dto,
            SectionDto {
                id: "id".to_string(),
                title: "title".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                tasks_preview: vec![TaskPreviewDto {
                    id: "id".to_string(),
                    title: "title".to_string(),
                    is_available: true,
                }],
            }
        );
    }
}
