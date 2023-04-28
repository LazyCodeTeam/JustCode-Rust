use content_domain::model::section_preview::SectionPreview;
use serde::{Deserialize, Serialize};

use crate::MapFrom;

#[derive(Debug, PartialEq, Eq, Clone, Default, Deserialize, Serialize)]
pub struct SectionPreviewDto {
    pub id: String,
    pub name: String,
}

impl MapFrom<SectionPreview> for SectionPreviewDto {
    fn map_from(model: SectionPreview) -> Self {
        Self {
            id: model.id,
            name: model.title,
        }
    }
}

impl MapFrom<SectionPreviewDto> for SectionPreview {
    fn map_from(dto: SectionPreviewDto) -> Self {
        Self {
            id: dto.id,
            title: dto.name,
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

        let section_preview_dto = SectionPreviewDto::map_from(section_preview);

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

        let section_preview = SectionPreview::map_from(section_preview_dto);

        assert_eq!(
            section_preview,
            SectionPreview {
                id: "id".to_string(),
                title: "name".to_string(),
            }
        );
    }
}
