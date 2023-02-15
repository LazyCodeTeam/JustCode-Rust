use super::section_dto::SectionDto;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate)]
pub struct TechnologyDto {
    #[validate(regex(path = "super::UUID_PATTERN", message = "Invalid UUID format"))]
    pub id: String,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    #[validate(url(message = "Invalid URL format"))]
    pub image: Option<String>,
    #[validate]
    pub sections: Vec<SectionDto>,
}
