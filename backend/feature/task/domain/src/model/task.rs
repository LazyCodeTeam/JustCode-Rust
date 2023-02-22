use super::task_content::TaskContent;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Task {
    pub id: String,
    pub section_id: String,
    pub title: String,
    pub difficulty: u8,
    pub dynamic: bool,
    pub for_anonymous: bool,
    pub content: TaskContent,
}
