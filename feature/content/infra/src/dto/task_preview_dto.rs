use content_domain::model::task_preview::TaskPreview;
use serde::{Deserialize, Serialize};

use crate::MapFrom;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TaskPreviewDto {
    pub id: String,
    pub title: String,
    pub for_anonymous: bool,
}

impl MapFrom<TaskPreview> for TaskPreviewDto {
    fn map_from(model: TaskPreview) -> Self {
        Self {
            id: model.id,
            title: model.title,
            for_anonymous: model.for_anonymous,
        }
    }
}

impl MapFrom<TaskPreviewDto> for TaskPreview {
    fn map_from(dto: TaskPreviewDto) -> Self {
        Self {
            id: dto.id,
            title: dto.title,
            for_anonymous: dto.for_anonymous,
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
            for_anonymous: false,
        };

        let task_preview_dto = TaskPreviewDto::map_from(task_preview);

        assert_eq!(
            task_preview_dto,
            TaskPreviewDto {
                id: "id".to_string(),
                title: "title".to_string(),
                for_anonymous: false,
            }
        );
    }

    #[test]
    fn from_task_preview_dto() {
        let task_preview_dto = TaskPreviewDto {
            id: "id".to_string(),
            title: "title".to_string(),
            for_anonymous: false,
        };

        let task_preview = TaskPreview::map_from(task_preview_dto);

        assert_eq!(
            task_preview,
            TaskPreview {
                id: "id".to_string(),
                title: "title".to_string(),
                for_anonymous: false,
            }
        );
    }
}
