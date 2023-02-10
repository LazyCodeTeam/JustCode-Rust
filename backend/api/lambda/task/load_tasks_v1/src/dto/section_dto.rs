use super::task_dto::TaskDto;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate)]
pub struct SectionDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub tasks: Vec<TaskDto>,
}
