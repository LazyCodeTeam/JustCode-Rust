use crate::{MapFrom, SectionPreviewDto};
use content_domain::model::section_preview::SectionPreview;

impl MapFrom<SectionPreview> for SectionPreviewDto {
    fn map_from(model: SectionPreview) -> Self {
        Self {
            id: model.id,
            title: model.title,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_section_preview() {
        let section_preview = SectionPreview {
            id: "id".to_string(),
            title: "title".to_string(),
        };

        let section_preview_dto = SectionPreviewDto::map_from(section_preview);

        assert_eq!(
            section_preview_dto,
            SectionPreviewDto {
                id: "id".to_string(),
                title: "title".to_string(),
            }
        );
    }
}
