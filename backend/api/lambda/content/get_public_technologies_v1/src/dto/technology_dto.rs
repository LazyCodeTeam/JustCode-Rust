use content_domain::model::technology::Technology;
use serde::{Deserialize, Serialize};

use super::section_preview_dto::SectionPreviewDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TechnologyDto {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections_preview: Vec<SectionPreviewDto>,
}

impl From<Technology> for TechnologyDto {
    fn from(technology: Technology) -> Self {
        Self {
            id: technology.id,
            name: technology.name,
            description: technology.description,
            image: technology.image,
            sections_preview: technology
                .sections_preview
                .into_iter()
                .map(SectionPreviewDto::from)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::section_preview::SectionPreview;

    use super::*;

    #[test]
    fn from_technology() {
        let section_previews = vec![SectionPreview::default()];

        let technology = Technology {
            id: "id".to_string(),
            name: "name".to_string(),
            description: Some("description".to_string()),
            image: Some("image".to_string()),
            position: 1,
            sections_preview: section_previews.clone(),
        };

        let technology_dto = TechnologyDto::from(technology);

        assert_eq!(
            technology_dto,
            TechnologyDto {
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: section_previews
                    .into_iter()
                    .map(SectionPreviewDto::from)
                    .collect(),
            }
        );
    }
}
