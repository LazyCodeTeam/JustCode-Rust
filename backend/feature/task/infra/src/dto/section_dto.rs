use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use serde::{Deserialize, Serialize};
use task_domain::model::section::Section;

use crate::{POSITIONED_ID_LENGTH, SECTION_ID_PREFIX, TECHNOLOGY_ID_PREFIX};

use super::task_preview_dto::TaskPreviewDto;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct SectionDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub technology_id: String,
    #[serde(rename = "LSI_1")]
    pub positioned_id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks_preview: Vec<TaskPreviewDto>,
}

impl DynamoDbIdentifiable for SectionDto {
    fn pk(&self) -> String {
        self.technology_id.clone()
    }

    fn sk(&self) -> String {
        self.id.clone()
    }
}

impl From<Section> for SectionDto {
    fn from(section: Section) -> Self {
        Self {
            id: format!("{}{}", SECTION_ID_PREFIX, section.id),
            technology_id: format!("{}{}", TECHNOLOGY_ID_PREFIX, section.technology_id),
            positioned_id: format!(
                "{}{:0>len$}",
                SECTION_ID_PREFIX,
                section.position,
                len = POSITIONED_ID_LENGTH
            ),
            title: section.title,
            description: section.description,
            image: section.image,
            tasks_preview: section.tasks_preview.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<SectionDto> for Section {
    fn from(section_dto: SectionDto) -> Self {
        Self {
            id: section_dto.id.replace(SECTION_ID_PREFIX, ""),
            technology_id: section_dto.technology_id.replace(TECHNOLOGY_ID_PREFIX, ""),
            title: section_dto.title,
            position: section_dto
                .positioned_id
                .replace(SECTION_ID_PREFIX, "")
                .parse()
                .unwrap_or_default(),
            description: section_dto.description,
            image: section_dto.image,
            tasks_preview: section_dto
                .tasks_preview
                .into_iter()
                .map(Into::into)
                .collect(),
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
            position: 1,
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            tasks_preview: tasks_preview.clone(),
        };

        let section_dto = SectionDto::from(section);

        assert_eq!(
            section_dto,
            SectionDto {
                id: "section-id".to_string(),
                technology_id: "technology-technology_id".to_string(),
                positioned_id: "section-00000000000000000000000000000001".to_string(),
                title: "title".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                tasks_preview: tasks_preview.into_iter().map(Into::into).collect(),
            }
        );
    }

    #[test]
    fn from_section_dto() {
        let tasks_preview = vec![TaskPreview::default()];

        let section_dto = SectionDto {
            id: "section-id".to_string(),
            technology_id: "technology-technology_id".to_string(),
            positioned_id: "section-00000000000000000000000000000001".to_string(),
            title: "title".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            tasks_preview: tasks_preview.clone().into_iter().map(Into::into).collect(),
        };

        let section = Section::from(section_dto);

        assert_eq!(
            section,
            Section {
                id: "id".to_string(),
                technology_id: "technology_id".to_string(),
                title: "title".to_string(),
                position: 1,
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                tasks_preview: tasks_preview.into_iter().map(Into::into).collect(),
            }
        );
    }
}
