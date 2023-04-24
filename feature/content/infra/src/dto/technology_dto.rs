use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use content_domain::model::technology::Technology;
use serde::{Deserialize, Serialize};

use crate::{
    FromDto, FromModel, IntoDto, IntoModel, POSITIONED_ID_LENGTH, TECHNOLOGY_ID_PREFIX,
    TECHNOLOGY_PK,
};

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

impl FromModel<Technology> for TechnologyDto {
    fn from_model(model: Technology) -> Self {
        Self {
            pk: TECHNOLOGY_PK.to_string(),
            id: format!("{}{}", TECHNOLOGY_ID_PREFIX, model.id),
            positioned_id: format!(
                "{}{:0>len$}",
                TECHNOLOGY_ID_PREFIX,
                model.position,
                len = POSITIONED_ID_LENGTH
            ),
            name: model.name,
            description: model.description,
            image: model.image,
            sections_preview: model.sections_preview.into_dto(),
        }
    }
}

impl FromDto<TechnologyDto> for Technology {
    fn from_dto(dto: TechnologyDto) -> Self {
        Self {
            id: dto.id.replace(TECHNOLOGY_ID_PREFIX, ""),
            position: dto
                .positioned_id
                .replace(TECHNOLOGY_ID_PREFIX, "")
                .parse()
                .unwrap_or_default(),
            name: dto.name,
            description: dto.description,
            image: dto.image,
            sections_preview: dto.sections_preview.into_model(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::section_preview::SectionPreview;

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

        let technology_dto = TechnologyDto::from_model(technology);

        assert_eq!(
            technology_dto,
            TechnologyDto {
                pk: TECHNOLOGY_PK.to_string(),
                id: "technology-id".to_string(),
                positioned_id: "technology-00000000000000000000000000000001".to_owned(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: sections_preview.into_dto(),
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

        let technology = Technology::from_dto(technology_dto);

        assert_eq!(
            technology,
            Technology {
                id: "id".to_string(),
                position: 1,
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: sections_preview.into_model(),
            }
        );
    }
}
