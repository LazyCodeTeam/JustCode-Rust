use content_domain::model::section_preview::SectionPreview;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SectionPreviewDto {
    pub id: String,
    pub title: String,
}

impl From<SectionPreview> for SectionPreviewDto {
    fn from(section_preview: SectionPreview) -> Self {
        Self {
            id: section_preview.id,
            title: section_preview.title,
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

        let section_preview_dto = SectionPreviewDto::from(section_preview);

        assert_eq!(
            section_preview_dto,
            SectionPreviewDto {
                id: "id".to_string(),
                title: "title".to_string(),
            }
        );
    }
}
