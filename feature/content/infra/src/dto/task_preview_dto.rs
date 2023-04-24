use content_domain::model::task_preview::TaskPreview;
use serde::{Deserialize, Serialize};

use crate::{FromDto, FromModel};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TaskPreviewDto {
    pub id: String,
    pub title: String,
    pub for_anonymous: bool,
}

impl FromModel<TaskPreview> for TaskPreviewDto {
    fn from_model(model: TaskPreview) -> Self {
        Self {
            id: model.id,
            title: model.title,
            for_anonymous: model.for_anonymous,
        }
    }
}

impl FromDto<TaskPreviewDto> for TaskPreview {
    fn from_dto(dto: TaskPreviewDto) -> Self {
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

        let task_preview_dto = TaskPreviewDto::from_model(task_preview);

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

        let task_preview = TaskPreview::from_dto(task_preview_dto);

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
