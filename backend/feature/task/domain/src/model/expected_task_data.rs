use super::task_content::TaskContent;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct ExpectedTaskData {
    pub id: String,
    pub title: String,
    pub content: TaskContent,
    pub difficulty: u8,
    pub dynamic: bool,
    pub for_anonymous: bool,
}
