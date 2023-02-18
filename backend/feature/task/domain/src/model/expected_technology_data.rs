use super::expected_section_data::ExpectedSectionData;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ExpectedTechnologyData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub sections: Vec<ExpectedSectionData>,
}
