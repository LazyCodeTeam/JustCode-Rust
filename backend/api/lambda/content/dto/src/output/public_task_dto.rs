use content_domain::model::task::Task;
use serde::{Deserialize, Serialize};

use super::task_content_dto::TaskContentDto;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublicTaskDto {
    Available {
        id: String,
        title: String,
        difficulty: u8,
        content: TaskContentDto,
    },
    NotAvailable {
        id: String,
        title: String,
        difficulty: u8,
    },
}

impl From<Task> for PublicTaskDto {
    fn from(value: Task) -> Self {
        if value.for_anonymous {
            Self::Available {
                id: value.id,
                title: value.title,
                difficulty: value.difficulty,
                content: value.content.into(),
            }
        } else {
            Self::NotAvailable {
                id: value.id,
                title: value.title,
                difficulty: value.difficulty,
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
            dynamic: false,
            section_id: "section_id".to_string(),
            position: 1,
            title: "title".to_string(),
            difficulty: 1,
            content: TaskContent::Empty,
            for_anonymous: true,
        };
        let public_task_dto = PublicTaskDto::from(task);
        assert_eq!(
            public_task_dto,
            PublicTaskDto::Available {
                id: "id".to_string(),
                title: "title".to_string(),
                difficulty: 1,
                content: TaskContentDto::Empty,
            }
        );
    }

    #[test]
    fn from_task_not_anonymous() {
        let task = Task {
            id: "id".to_string(),
            dynamic: false,
            section_id: "section_id".to_string(),
            position: 1,
            title: "title".to_string(),
            difficulty: 1,
            content: TaskContent::Empty,
            for_anonymous: false,
        };
        let public_task_dto = PublicTaskDto::from(task);
        assert_eq!(
            public_task_dto,
            PublicTaskDto::NotAvailable {
                id: "id".to_string(),
                title: "title".to_string(),
                difficulty: 1,
            }
        );
    }
}
