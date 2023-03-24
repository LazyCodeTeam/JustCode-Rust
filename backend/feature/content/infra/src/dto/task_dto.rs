use common_infra::dynamodb_identifiable::DynamoDbIdentifiable;
use content_domain::model::task::Task;
use serde::{Deserialize, Serialize};

use crate::{
    DYNAMIC_TASK_ID_PREFIX, POSITIONED_ID_LENGTH, SECTION_ID_PREFIX, TASK_GSI_PK, TASK_ID_PREFIX,
};

use super::task_content_dto::TaskContentDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize, Default)]
pub struct TaskDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub section_id: String,
    #[serde(rename = "LSI_1")]
    pub lsi: String,
    pub title: String,
    pub difficulty: u8,
    pub for_anonymous: bool,
    pub content: TaskContentDto,
    #[serde(rename = "GSI_1_PK")]
    pub gsi_pk: String,
    #[serde(rename = "GSI_1_SK")]
    pub gsi_sk: String,
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
        let lsi = match task.position {
            None => format!("{}{}", DYNAMIC_TASK_ID_PREFIX, task.id),
            Some(position) => format!(
                "{}{:0>len$}",
                TASK_ID_PREFIX,
                position,
                len = POSITIONED_ID_LENGTH,
            ),
        };

        Self {
            id: format!("{}{}", TASK_ID_PREFIX, task.id),
            section_id: format!("{}{}", SECTION_ID_PREFIX, task.section_id),
            lsi,
            title: task.title,
            difficulty: task.difficulty,
            for_anonymous: task.for_anonymous,
            content: task.content.into(),
            gsi_pk: TASK_GSI_PK.to_string(),
            gsi_sk: format!("{}{}", TASK_ID_PREFIX, task.id),
        }
    }
}

impl From<TaskDto> for Task {
    fn from(task_dto: TaskDto) -> Self {
        Self {
            id: task_dto.id.replace(TASK_ID_PREFIX, ""),
            section_id: task_dto.section_id.replace(SECTION_ID_PREFIX, ""),
            title: task_dto.title,
            position: task_dto.lsi.replace(TASK_ID_PREFIX, "").parse::<u64>().ok(),
            difficulty: task_dto.difficulty,
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
            position: Some(2),
            difficulty: 1,
            for_anonymous: true,
            content: content.clone(),
        };

        let task_dto = TaskDto::from(task);

        assert_eq!(
            task_dto,
            TaskDto {
                id: "task-id".to_string(),
                section_id: "section-section_id".to_string(),
                lsi: "task-00000000000000000000000000000002".to_string(),
                title: "title".to_string(),
                difficulty: 1,
                for_anonymous: true,
                content: content.into(),
                gsi_pk: TASK_GSI_PK.to_string(),
                gsi_sk: "task-id".to_string(),
            }
        );
    }

    #[test]
    fn from_dynamic_task() {
        let content = TaskContent::default();
        let task = Task {
            id: "id".to_string(),
            section_id: "section_id".to_string(),
            title: "title".to_string(),
            position: None,
            difficulty: 1,
            for_anonymous: true,
            content: content.clone(),
        };

        let task_dto = TaskDto::from(task);

        assert_eq!(
            task_dto,
            TaskDto {
                id: "task-id".to_string(),
                section_id: "section-section_id".to_string(),
                lsi: "dynamic_task-id".to_string(),
                title: "title".to_string(),
                difficulty: 1,
                for_anonymous: true,
                content: content.into(),
                gsi_pk: TASK_GSI_PK.to_string(),
                gsi_sk: "task-id".to_string(),
            }
        );
    }

    #[test]
    fn from_task_dto() {
        let content = TaskContent::default();
        let task_dto = TaskDto {
            id: "task-id".to_string(),
            section_id: "section-section_id".to_string(),
            lsi: "task-00000000000000000000000000000002".to_string(),
            title: "title".to_string(),
            difficulty: 1,
            for_anonymous: true,
            content: content.clone().into(),
            gsi_pk: TASK_GSI_PK.to_string(),
            gsi_sk: "task-id".to_string(),
        };

        let task = Task::from(task_dto);

        assert_eq!(
            task,
            Task {
                id: "id".to_string(),
                section_id: "section_id".to_string(),
                title: "title".to_string(),
                position: Some(2),
                difficulty: 1,
                for_anonymous: true,
                content,
            }
        );
    }

    #[test]
    fn from_dynamic_task_dto() {
        let content = TaskContent::default();
        let task_dto = TaskDto {
            id: "task-id".to_string(),
            section_id: "section-section_id".to_string(),
            lsi: "task-aaaa".to_string(),
            title: "title".to_string(),
            difficulty: 1,
            for_anonymous: true,
            content: content.clone().into(),
            gsi_pk: TASK_GSI_PK.to_string(),
            gsi_sk: "task-id".to_string(),
        };

        let task = Task::from(task_dto);

        assert_eq!(
            task,
            Task {
                id: "id".to_string(),
                section_id: "section_id".to_string(),
                title: "title".to_string(),
                position: None,
                difficulty: 1,
                for_anonymous: true,
                content,
            }
        );
    }
}
