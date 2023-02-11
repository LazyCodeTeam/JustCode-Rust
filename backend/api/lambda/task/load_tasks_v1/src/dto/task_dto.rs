use super::task_content_dto::TaskContentDto;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate)]
pub struct TaskDto {
    pub id: String,
    pub title: String,
    #[validate(range(min = 1, max = 10))]
    pub difficulty: u8,
    pub content: TaskContentDto,
}
