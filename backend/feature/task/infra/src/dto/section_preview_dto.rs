use serde::{Deserialize, Serialize};
use task_domain::model::section_preview::SectionPreview;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct SectionPreviewDto {
    pub id: String,
    pub name: String,
}

impl From<SectionPreview> for SectionPreviewDto {
    fn from(section_preview: SectionPreview) -> Self {
        Self {
            id: section_preview.id,
            name: section_preview.name,
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
            name: "name".to_string(),
        };

        let section_preview_dto = SectionPreviewDto::from(section_preview);

        assert_eq!(
            section_preview_dto,
            SectionPreviewDto {
                id: "id".to_string(),
                name: "name".to_string(),
            }
        );
    }
}
