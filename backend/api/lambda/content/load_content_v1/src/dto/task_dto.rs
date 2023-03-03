use content_domain::model::expected_task_data::ExpectedTaskData;
#[cfg(feature = "fake_dto")]
use fake::{Dummy, Fake};
use serde::Deserialize;
use uuid::Uuid;
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
#[cfg_attr(feature = "fake_dto", derive(Dummy, serde::Serialize))]
pub struct TaskDto {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub title: String,
    pub content: TaskContentDto,
    #[dummy(faker = "1..10")]
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
            id: value.id.simple().to_string(),
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
        let uuid = Uuid::new_v4();
        let task = TaskDto {
            id: uuid,
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
                id: uuid.simple().to_string(),
                title: "title".to_string(),
                content: content.into(),
                difficulty: 1,
                dynamic: false,
                for_anonymous: false,
            }
        );
    }
}
