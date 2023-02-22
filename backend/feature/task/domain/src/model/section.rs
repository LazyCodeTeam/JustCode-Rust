use super::task_preview::TaskPreview;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Section {
    pub id: String,
    pub technology_id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks_preview: Vec<TaskPreview>,
}
