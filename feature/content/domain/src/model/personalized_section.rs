use super::personalized_task_preview::PersonalizedTaskPreview;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PersonalizedSection {
    pub id: String,
    pub technology_id: String,
    pub position: u64,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks_preview: Vec<PersonalizedTaskPreview>,
}
