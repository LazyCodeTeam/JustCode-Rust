use crate::{FromModel, PersonalizedTaskDto};
use content_domain::model::personalized_task::PersonalizedTask;
use gen::models::TaskContentDto;

impl FromModel<PersonalizedTask> for PersonalizedTaskDto {
    fn from_model(model: PersonalizedTask) -> Self {
        Self {
            id: model.id,
            title: model.title,
            difficulty: model.difficulty,
            done_at: model.done_at.map(|date| date.to_rfc3339()),
            content: Option::<TaskContentDto>::from_model(model.content).map(Box::new),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::task_content::TaskContent;

    use super::*;

    #[test]
    fn from_models_vec() {
        let now = chrono::Utc::now();
        let models = vec![
            PersonalizedTask {
                id: "id".to_owned(),
                title: "title".to_owned(),
                difficulty: 1,
                done_at: Some(now),
                for_anonymous: false,
                section_id: "section_id".to_owned(),
                position: Option::None,
                content: TaskContent::Empty,
            },
            PersonalizedTask {
                id: "id_2".to_owned(),
                title: "title_2".to_owned(),
                difficulty: 2,
                done_at: None,
                for_anonymous: false,
                section_id: "section_id_2".to_owned(),
                position: Option::None,
                content: TaskContent::Empty,
            },
        ];

        let dtos = Vec::<PersonalizedTaskDto>::from_model(models);

        assert_eq!(dtos.len(), 2);
        assert_eq!(dtos[0].id, "id");
        assert_eq!(dtos[0].title, "title");
        assert_eq!(dtos[0].difficulty, 1);
        assert_eq!(dtos[0].done_at, Some(now.to_rfc3339()));
        assert!(dtos[0].content.is_none());
        assert_eq!(dtos[1].id, "id_2");
        assert_eq!(dtos[1].title, "title_2");
        assert_eq!(dtos[1].difficulty, 2);
        assert_eq!(dtos[1].done_at, None);
        assert!(dtos[1].content.is_none());
    }

    #[test]
    fn from_model() {
        let now = chrono::Utc::now();
        let model = PersonalizedTask {
            id: "id".to_owned(),
            title: "title".to_owned(),
            difficulty: 1,
            done_at: Some(now),
            for_anonymous: false,
            section_id: "section_id".to_owned(),
            position: Option::None,
            content: TaskContent::Empty,
        };

        let dto = PersonalizedTaskDto::from_model(model);

        assert_eq!(dto.id, "id");
        assert_eq!(dto.title, "title");
        assert_eq!(dto.difficulty, 1);
        assert_eq!(dto.done_at, Some(now.to_rfc3339()));
        assert!(dto.content.is_none());
    }
}
