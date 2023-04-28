use content_domain::model::expected_task_data::ExpectedTaskData;

use crate::{ExpectedTaskDto, MapFrom, MapInto};

const DEFAULT_DIFFICULTY: u8 = 1;
const DEFAULT_DYNAMIC: bool = false;
const DEFAULT_FOR_ANONYMOUS: bool = false;

impl MapFrom<ExpectedTaskDto> for ExpectedTaskData {
    fn map_from(dto: ExpectedTaskDto) -> Self {
        ExpectedTaskData {
            id: dto.id.simple().to_string(),
            title: dto.title,
            content: dto.content.map(|content| *content).map_into(),
            difficulty: dto.difficulty.unwrap_or(DEFAULT_DIFFICULTY),
            dynamic: dto.dynamic.unwrap_or(DEFAULT_DYNAMIC),
            for_anonymous: dto.for_anonymous.unwrap_or(DEFAULT_FOR_ANONYMOUS),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ExpectedTaskContentDto;
    use uuid::Uuid;

    #[test]
    fn test_from_with_defaults() {
        let uuid = Uuid::new_v4();
        let content = ExpectedTaskContentDto::TaskContentLessonDto {
            content: "content".to_string(),
        };
        let task = ExpectedTaskDto {
            id: uuid,
            title: "title".to_string(),
            content: Some(Box::new(content.clone())),
            difficulty: None,
            dynamic: None,
            for_anonymous: None,
        };

        let expected_task_data: ExpectedTaskData = task.map_into();

        assert_eq!(
            expected_task_data,
            ExpectedTaskData {
                id: uuid.simple().to_string(),
                title: "title".to_string(),
                content: content.map_into(),
                difficulty: 1,
                dynamic: false,
                for_anonymous: false,
            }
        );
    }

    #[test]
    fn test_from_with_values() {
        let uuid = Uuid::new_v4();
        let content = ExpectedTaskContentDto::TaskContentLessonDto {
            content: "content".to_string(),
        };
        let task = ExpectedTaskDto {
            id: uuid,
            title: "title".to_string(),
            content: Some(Box::new(content.clone())),
            difficulty: Some(2),
            dynamic: Some(true),
            for_anonymous: Some(true),
        };

        let expected_task_data: ExpectedTaskData = task.map_into();

        assert_eq!(
            expected_task_data,
            ExpectedTaskData {
                id: uuid.simple().to_string(),
                title: "title".to_string(),
                content: content.map_into(),
                difficulty: 2,
                dynamic: true,
                for_anonymous: true,
            }
        );
    }
}
