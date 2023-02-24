use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use serde::{Deserialize, Serialize};
use task_domain::model::technology::Technology;

use crate::{POSITIONED_ID_LENGTH, TECHNOLOGY_ID_PREFIX, TECHNOLOGY_PK};

use super::section_preview_dto::SectionPreviewDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
pub struct TechnologyDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "LSI_1")]
    pub positioned_id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections_preview: Vec<SectionPreviewDto>,
}

impl DynamoDbIdentifiable for TechnologyDto {
    fn pk(&self) -> String {
        self.pk.clone()
    }

    fn sk(&self) -> String {
        self.id.clone()
    }
}

impl From<Technology> for TechnologyDto {
    fn from(technology: Technology) -> Self {
        Self {
            pk: TECHNOLOGY_PK.to_string(),
            id: format!("{}{}", TECHNOLOGY_ID_PREFIX, technology.id),
            positioned_id: format!(
                "{}{:0>len$}",
                TECHNOLOGY_ID_PREFIX,
                technology.position,
                len = POSITIONED_ID_LENGTH
            ),
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
            id: technology_dto.id.replace(TECHNOLOGY_ID_PREFIX, ""),
            position: technology_dto
                .positioned_id
                .replace(TECHNOLOGY_ID_PREFIX, "")
                .parse()
                .unwrap_or_default(),
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
            id: "id".to_string(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            position: 1,
            image: Some("image".to_string()),
            sections_preview: sections_preview.clone(),
        };

        let technology_dto = TechnologyDto::from(technology);

        assert_eq!(
            technology_dto,
            TechnologyDto {
                pk: TECHNOLOGY_PK.to_string(),
                id: "technology-id".to_string(),
                positioned_id: "technology-00000000000000000000000000000001".to_owned(),
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
            id: "technology-id".to_string(),
            positioned_id: "technology-00000000000000000000000000000001".to_owned(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            sections_preview: sections_preview.clone(),
        };

        let technology = Technology::from(technology_dto);

        assert_eq!(
            technology,
            Technology {
                id: "id".to_string(),
                position: 1,
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: sections_preview.into_iter().map(Into::into).collect(),
            }
        );
    }
}
