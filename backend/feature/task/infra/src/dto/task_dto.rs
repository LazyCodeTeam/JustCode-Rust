use serde::{Deserialize, Serialize};

use super::task_content_dto::TaskContentDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TaskDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub section_id: String,
    pub title: String,
    pub difficulty: u8,
    pub dynamic: bool,
    pub for_anonymous: bool,
    pub content: TaskContentDto,
}
