use super::section_preview::SectionPreview;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Technology {
    pub pk: String,
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections_preview: Vec<SectionPreview>,
}
