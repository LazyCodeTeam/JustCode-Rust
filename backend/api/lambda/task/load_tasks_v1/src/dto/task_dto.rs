use serde::Deserialize;
use validator::Validate;

use super::task_content_dto::TaskContentDto;

const fn default_difficulty() -> u8 {
    1
}

const fn default_dynamic() -> bool {
    false
}

const fn default_for_anonymous() -> bool {
    false
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate)]
pub struct TaskDto {
    #[validate(regex(path = "super::UUID_PATTERN", message = "Invalid UUID format"))]
    pub id: String,
    #[validate(length(min = 1))]
    pub title: String,
    pub content: TaskContentDto,
    #[validate(range(min = 1, max = 10))]
    #[serde(default = "default_difficulty")]
    pub difficulty: u8,
    #[serde(default = "default_dynamic")]
    pub dynamic: bool,
    #[serde(default = "default_for_anonymous")]
    pub for_anonymous: bool,
}
