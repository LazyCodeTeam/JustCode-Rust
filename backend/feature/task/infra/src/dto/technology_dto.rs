use serde::{Deserialize, Serialize};
use task_domain::model::technology::Technology;

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

impl From<Technology> for TechnologyDto {
    fn from(technology: Technology) -> Self {
        Self {
            pk: technology.pk,
            id: technology.id,
            name: technology.name,
            description: technology.description,
            image: technology.image,
            sections_preview: technology
                .sections_preview
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<TechnologyDto> for Technology {
    fn from(technology_dto: TechnologyDto) -> Self {
        Self {
            pk: technology_dto.pk,
            id: technology_dto.id,
            name: technology_dto.name,
            description: technology_dto.description,
            image: technology_dto.image,
            sections_preview: technology_dto
                .sections_preview
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use task_domain::model::section_preview::SectionPreview;

    use super::*;

    #[test]
    fn from_technology() {
        let sections_preview = vec![SectionPreview::default()];

        let technology = Technology {
            pk: "pk".to_string(),
            id: "id".to_string(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            sections_preview: sections_preview.clone(),
        };

        let technology_dto = TechnologyDto::from(technology);

        assert_eq!(
            technology_dto,
            TechnologyDto {
                pk: "pk".to_string(),
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: sections_preview.into_iter().map(Into::into).collect(),
            }
        );
    }

    #[test]
    fn from_technology_dto() {
        let sections_preview = vec![SectionPreviewDto::default()];

        let technology_dto = TechnologyDto {
            pk: "pk".to_string(),
            id: "id".to_string(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            sections_preview: sections_preview.clone(),
        };

        let technology = Technology::from(technology_dto);

        assert_eq!(
            technology,
            Technology {
                pk: "pk".to_string(),
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: sections_preview.into_iter().map(Into::into).collect(),
            }
        );
    }
}
