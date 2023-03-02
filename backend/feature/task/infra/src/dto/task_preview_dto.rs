use serde::{Deserialize, Serialize};
use task_domain::model::task_preview::TaskPreview;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TaskPreviewDto {
    pub id: String,
    pub title: String,
}

impl From<TaskPreview> for TaskPreviewDto {
    fn from(task_preview: TaskPreview) -> Self {
        Self {
            id: task_preview.id,
            title: task_preview.title,
        }
    }
}

impl From<TaskPreviewDto> for TaskPreview {
    fn from(task_preview_dto: TaskPreviewDto) -> Self {
        Self {
            id: task_preview_dto.id,
            title: task_preview_dto.title,
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
        };

        let task_preview_dto = TaskPreviewDto::from(task_preview);

        assert_eq!(
            task_preview_dto,
            TaskPreviewDto {
                id: "id".to_string(),
                title: "title".to_string(),
            }
        );
    }

    #[test]
    fn from_task_preview_dto() {
        let task_preview_dto = TaskPreviewDto {
            id: "id".to_string(),
            title: "title".to_string(),
        };

        let task_preview = TaskPreview::from(task_preview_dto);

        assert_eq!(
            task_preview,
            TaskPreview {
                id: "id".to_string(),
                title: "title".to_string(),
            }
        );
    }
}
