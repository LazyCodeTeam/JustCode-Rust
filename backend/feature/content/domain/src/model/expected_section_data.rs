use super::expected_task_data::ExpectedTaskData;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ExpectedSectionData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks: Vec<ExpectedTaskData>,
}
