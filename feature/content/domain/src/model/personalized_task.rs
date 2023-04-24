use chrono::{DateTime, Utc};

use super::task_content::TaskContent;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PersonalizedTask {
    pub id: String,
    pub section_id: String,
    pub position: Option<u64>,
    pub title: String,
    pub difficulty: u8,
    pub for_anonymous: bool,
    pub done_at: Option<DateTime<Utc>>,
    pub content: TaskContent,
}
