use super::task_dto::TaskDto;
use content_domain::model::expected_section_data::ExpectedSectionData;
#[cfg(feature = "fake_dto")]
use fake::{Dummy, Fake, Faker};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate, Default)]
#[cfg_attr(feature = "fake_dto", derive(Dummy, serde::Serialize))]
pub struct SectionDto {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    #[validate(url(message = "Invalid URL format"))]
    pub image: Option<String>,
    #[validate]
    #[dummy(faker = "(Faker, 3..10)")]
    pub tasks: Vec<TaskDto>,
}

impl From<SectionDto> for ExpectedSectionData {
    fn from(value: SectionDto) -> Self {
        ExpectedSectionData {
            id: value.id.simple().to_string(),
            title: value.title,
            description: value.description,
            image: value.image,
            tasks: value.tasks.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let task = TaskDto::default();
        let uuid = Uuid::new_v4();
        let section = SectionDto {
            id: uuid,
            title: "title".to_string(),
            description: None,
            image: None,
            tasks: vec![task.clone()],
        };
        let expected = ExpectedSectionData {
            id: uuid.simple().to_string(),
            title: "title".to_string(),
            description: None,
            image: None,
            tasks: vec![task.into()],
        };

        let result = ExpectedSectionData::from(section);

        assert_eq!(result, expected);
    }
}
