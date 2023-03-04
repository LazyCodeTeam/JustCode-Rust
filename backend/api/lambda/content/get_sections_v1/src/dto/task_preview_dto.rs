use content_domain::model::task_preview::TaskPreview;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TaskPreviewDto {
    pub id: String,
    pub title: String,
    pub is_available: bool,
}

impl From<TaskPreview> for TaskPreviewDto {
    fn from(task_preview: TaskPreview) -> Self {
        Self {
            id: task_preview.id,
            title: task_preview.title,
            is_available: task_preview.for_anonymous,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_task_preview() {
        let task_preview = TaskPreview {
            id: "id".to_string(),
            title: "title".to_string(),
            for_anonymous: true,
        };

        let task_preview_dto = TaskPreviewDto::from(task_preview);

        assert_eq!(
            task_preview_dto,
            TaskPreviewDto {
                id: "id".to_string(),
                title: "title".to_string(),
                is_available: true,
            }
        );
    }
}
