use crate::{MapFrom, MapInto, SectionDto};
use content_domain::model::section::Section;

impl MapFrom<Section> for SectionDto {
    fn map_from(model: Section) -> Self {
        Self {
            id: model.id,
            title: model.title,
            description: model.description,
            image: model.image,
            tasks_preview: model
                .tasks_preview
                .into_iter()
                .map(MapInto::map_into)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TaskPreviewDto;
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

        let section_dto = SectionDto::map_from(section);

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
