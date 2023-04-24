use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use content_domain::model::section::Section;
use serde::{Deserialize, Serialize};

use crate::{
    FromDto, FromModel, IntoDto, IntoModel, POSITIONED_ID_LENGTH, SECTION_ID_PREFIX,
    TECHNOLOGY_ID_PREFIX,
};

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

impl FromModel<Section> for SectionDto {
    fn from_model(model: Section) -> Self {
        Self {
            id: format!("{}{}", SECTION_ID_PREFIX, model.id),
            technology_id: format!("{}{}", TECHNOLOGY_ID_PREFIX, model.technology_id),
            positioned_id: format!(
                "{}{:0>len$}",
                SECTION_ID_PREFIX,
                model.position,
                len = POSITIONED_ID_LENGTH
            ),
            title: model.title,
            description: model.description,
            image: model.image,
            tasks_preview: model.tasks_preview.into_dto(),
        }
    }
}

impl FromDto<SectionDto> for Section {
    fn from_dto(dto: SectionDto) -> Self {
        Self {
            id: dto.id.replace(SECTION_ID_PREFIX, ""),
            technology_id: dto.technology_id.replace(TECHNOLOGY_ID_PREFIX, ""),
            title: dto.title,
            position: dto
                .positioned_id
                .replace(SECTION_ID_PREFIX, "")
                .parse()
                .unwrap_or_default(),
            description: dto.description,
            image: dto.image,
            tasks_preview: dto.tasks_preview.into_model(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::task_preview::TaskPreview;

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

        let section_dto = SectionDto::from_model(section);

        assert_eq!(
            section_dto,
            SectionDto {
                id: "section-id".to_string(),
                technology_id: "technology-technology_id".to_string(),
                positioned_id: "section-00000000000000000000000000000001".to_string(),
                title: "title".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                tasks_preview: tasks_preview.into_dto(),
            }
        );
    }

    #[test]
    fn from_section_dto() {
        let tasks_preview = vec![TaskPreviewDto {
            id: "task-id".to_string(),
            title: "title".to_string(),
            for_anonymous: true,
        }];

        let section_dto = SectionDto {
            id: "section-id".to_string(),
            technology_id: "technology-technology_id".to_string(),
            positioned_id: "section-00000000000000000000000000000001".to_string(),
            title: "title".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            tasks_preview: tasks_preview.clone(),
        };

        let section = Section::from_dto(section_dto);

        assert_eq!(
            section,
            Section {
                id: "id".to_string(),
                technology_id: "technology_id".to_string(),
                title: "title".to_string(),
                position: 1,
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                tasks_preview: tasks_preview.into_model(),
            }
        );
    }
}
