use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use content_domain::model::task::Task;
use serde::{Deserialize, Serialize};

use crate::{POSITIONED_ID_LENGTH, SECTION_ID_PREFIX, TASK_ID_PREFIX};

use super::task_content_dto::TaskContentDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
pub struct TaskDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub section_id: String,
    #[serde(rename = "LSI_1")]
    pub positioned_id: String,
    pub title: String,
    pub difficulty: u8,
    pub dynamic: bool,
    pub for_anonymous: bool,
    pub content: TaskContentDto,
}

impl DynamoDbIdentifiable for TaskDto {
    fn pk(&self) -> String {
        self.section_id.clone()
    }

    fn sk(&self) -> String {
        self.id.clone()
    }
}

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: format!("{}{}", TASK_ID_PREFIX, task.id),
            section_id: format!("{}{}", SECTION_ID_PREFIX, task.section_id),
            positioned_id: format!(
                "{}{:0>len$}",
                TASK_ID_PREFIX,
                task.position,
                len = POSITIONED_ID_LENGTH,
            ),
            title: task.title,
            difficulty: task.difficulty,
            dynamic: task.dynamic,
            for_anonymous: task.for_anonymous,
            content: task.content.into(),
        }
    }
}

impl From<TaskDto> for Task {
    fn from(task_dto: TaskDto) -> Self {
        Self {
            id: task_dto.id.replace(TASK_ID_PREFIX, ""),
            section_id: task_dto.section_id.replace(SECTION_ID_PREFIX, ""),
            title: task_dto.title,
            position: task_dto
                .positioned_id
                .replace(TASK_ID_PREFIX, "")
                .parse()
                .unwrap_or_default(),
            difficulty: task_dto.difficulty,
            dynamic: task_dto.dynamic,
            for_anonymous: task_dto.for_anonymous,
            content: task_dto.content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::task_content::TaskContent;

    use super::*;

    #[test]
    fn from_task() {
        let content = TaskContent::default();
        let task = Task {
            id: "id".to_string(),
            section_id: "section_id".to_string(),
            title: "title".to_string(),
            position: 2,
            difficulty: 1,
            dynamic: true,
            for_anonymous: true,
            content: content.clone(),
        };

        let task_dto = TaskDto::from(task);

        assert_eq!(
            task_dto,
            TaskDto {
                id: "task-id".to_string(),
                section_id: "section-section_id".to_string(),
                positioned_id: "task-00000000000000000000000000000002".to_string(),
                title: "title".to_string(),
                difficulty: 1,
                dynamic: true,
                for_anonymous: true,
                content: content.into(),
            }
        );
    }

    #[test]
    fn into_task() {
        let content = TaskContent::default();
        let task_dto = TaskDto {
            id: "task-id".to_string(),
            section_id: "section-section_id".to_string(),
            positioned_id: "task-00000000000000000000000000000002".to_string(),
            title: "title".to_string(),
            difficulty: 1,
            dynamic: true,
            for_anonymous: true,
            content: content.clone().into(),
        };

        let task = Task::from(task_dto);

        assert_eq!(
            task,
            Task {
                id: "id".to_string(),
                section_id: "section_id".to_string(),
                title: "title".to_string(),
                position: 2,
                difficulty: 1,
                dynamic: true,
                for_anonymous: true,
                content,
            }
        );
    }
}
