use serde::{Deserialize, Serialize};
use task_domain::model::task::Task;

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

impl From<Task> for TaskDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            section_id: task.section_id,
            title: task.title,
            difficulty: task.difficulty,
            dynamic: task.dynamic,
            for_anonymous: task.for_anonymous,
            content: task.content.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use task_domain::model::task_content::TaskContent;

    use super::*;

    #[test]
    fn from_task() {
        let content = TaskContent::default();
        let task = Task {
            id: "id".to_string(),
            section_id: "section_id".to_string(),
            title: "title".to_string(),
            difficulty: 1,
            dynamic: true,
            for_anonymous: true,
            content: content.clone(),
        };

        let task_dto = TaskDto::from(task);

        assert_eq!(
            task_dto,
            TaskDto {
                id: "id".to_string(),
                section_id: "section_id".to_string(),
                title: "title".to_string(),
                difficulty: 1,
                dynamic: true,
                for_anonymous: true,
                content: content.into(),
            }
        );
    }
}
