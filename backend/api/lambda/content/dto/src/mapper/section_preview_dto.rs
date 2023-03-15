use crate::{FromModel, SectionPreviewDto};
use content_domain::model::section_preview::SectionPreview;

impl FromModel<SectionPreview> for SectionPreviewDto {
    fn from_model(model: SectionPreview) -> Self {
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

        let section_preview_dto = SectionPreviewDto::from_model(section_preview);

        assert_eq!(
            section_preview_dto,
            SectionPreviewDto {
                id: "id".to_string(),
                title: "title".to_string(),
            }
        );
    }
}
