use content_domain::model::technology::Technology;

use crate::{FromModel, IntoDto, TechnologyDto};

impl FromModel<Technology> for TechnologyDto {
    fn from_model(model: Technology) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            image: model.image,
            sections_preview: model
                .sections_preview
                .into_iter()
                .map(IntoDto::into_dto)
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

        let technology_dto = TechnologyDto::from_model(technology);

        assert_eq!(
            technology_dto,
            TechnologyDto {
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: section_previews
                    .into_iter()
                    .map(IntoDto::into_dto)
                    .collect(),
            }
        );
    }
}
