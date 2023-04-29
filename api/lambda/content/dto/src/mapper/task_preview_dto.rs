use crate::{MapFrom, TaskPreviewDto};
use content_domain::model::task_preview::TaskPreview;

impl MapFrom<TaskPreview> for TaskPreviewDto {
    fn map_from(model: TaskPreview) -> Self {
        Self {
            id: model.id,
            title: model.title,
            is_available: model.for_anonymous,
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

        let task_preview_dto = TaskPreviewDto::map_from(task_preview);

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
