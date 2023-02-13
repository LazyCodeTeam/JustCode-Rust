use serde::Deserialize;
use validator::Validate;

use super::task_content_dto::TaskContentDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate)]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    pub content: TaskContentDto,
    pub difficulty: u8,
    pub dynamic: bool,
}
