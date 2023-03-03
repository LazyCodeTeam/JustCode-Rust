use content_domain::model::section_preview::SectionPreview;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct SectionPreviewDto {
    pub id: String,
    pub name: String,
}

impl From<SectionPreview> for SectionPreviewDto {
    fn from(section_preview: SectionPreview) -> Self {
        Self {
            id: section_preview.id,
            name: section_preview.title,
        }
    }
}

impl From<SectionPreviewDto> for SectionPreview {
    fn from(section_preview_dto: SectionPreviewDto) -> Self {
        Self {
            id: section_preview_dto.id,
            title: section_preview_dto.name,
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
            title: "name".to_string(),
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

    #[test]
    fn from_section_preview_dto() {
        let section_preview_dto = SectionPreviewDto {
            id: "id".to_string(),
            name: "name".to_string(),
        };

        let section_preview = SectionPreview::from(section_preview_dto);

        assert_eq!(
            section_preview,
            SectionPreview {
                id: "id".to_string(),
                title: "name".to_string(),
            }
        );
    }
}
