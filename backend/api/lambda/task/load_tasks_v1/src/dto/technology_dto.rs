use super::section_dto::SectionDto;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate)]
pub struct TechnologyDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections: Vec<SectionDto>,
}
