use crate::{FromModel, PublicTaskDto, TaskContentDto};
use content_domain::model::task::Task;

impl FromModel<Task> for PublicTaskDto {
    fn from_model(model: Task) -> Self {
        if model.for_anonymous {
            Self::PublicTaskAvailableDto {
                id: model.id,
                title: model.title,
                difficulty: model.difficulty,
                content: Option::<TaskContentDto>::from_model(model.content).map(Box::new),
            }
        } else {
            Self::PublicTaskNotAvailableDto {
                id: model.id,
                title: model.title,
                difficulty: model.difficulty,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::task_content::TaskContent;

    use super::*;

    #[test]
    fn from_task_anonymous() {
        let task = Task {
            id: "id".to_string(),
            section_id: "section_id".to_string(),
            position: Some(1),
            title: "title".to_string(),
            difficulty: 1,
            content: TaskContent::Empty,
            for_anonymous: true,
        };
        let public_task_dto = PublicTaskDto::from_model(task);
        assert_eq!(
            public_task_dto,
            PublicTaskDto::PublicTaskAvailableDto {
                id: "id".to_string(),
                title: "title".to_string(),
                difficulty: 1,
                content: None,
            }
        );
    }

    #[test]
    fn from_task_not_anonymous() {
        let task = Task {
            id: "id".to_string(),
            section_id: "section_id".to_string(),
            position: Some(1),
            title: "title".to_string(),
            difficulty: 1,
            content: TaskContent::Empty,
            for_anonymous: false,
        };
        let public_task_dto = PublicTaskDto::from_model(task);
        assert_eq!(
            public_task_dto,
            PublicTaskDto::PublicTaskNotAvailableDto {
                id: "id".to_string(),
                title: "title".to_string(),
                difficulty: 1,
            }
        );
    }
}
