use content_domain::model::technology::Technology;

use crate::{MapFrom, MapInto, TechnologyDto};

impl MapFrom<Technology> for TechnologyDto {
    fn map_from(model: Technology) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            image: model.image,
            sections_preview: model
                .sections_preview
                .into_iter()
                .map(MapInto::map_into)
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

        let technology_dto = TechnologyDto::map_from(technology);

        assert_eq!(
            technology_dto,
            TechnologyDto {
                id: "id".to_string(),
                name: "name".to_string(),
                description: Some("description".to_string()),
                image: Some("image".to_string()),
                sections_preview: section_previews
                    .into_iter()
                    .map(MapInto::map_into)
                    .collect(),
            }
        );
    }
}
