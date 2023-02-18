use serde::Deserialize;
use task_domain::model::expected_task_data::ExpectedTaskData;
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

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate, Default)]
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

impl From<TaskDto> for ExpectedTaskData {
    fn from(value: TaskDto) -> Self {
        ExpectedTaskData {
            id: value.id,
            title: value.title,
            content: value.content.into(),
            difficulty: value.difficulty,
            dynamic: value.dynamic,
            for_anonymous: value.for_anonymous,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let content = TaskContentDto::default();
        let task = TaskDto {
            id: "id".to_string(),
            title: "title".to_string(),
            content: content.clone(),
            difficulty: 1,
            dynamic: false,
            for_anonymous: false,
        };

        let expected_task_data: ExpectedTaskData = task.into();

        assert_eq!(
            expected_task_data,
            ExpectedTaskData {
                id: "id".to_string(),
                title: "title".to_string(),
                content: content.into(),
                difficulty: 1,
                dynamic: false,
                for_anonymous: false,
            }
        );
    }
}
