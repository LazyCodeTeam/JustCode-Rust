use super::task_dto::TaskDto;
use serde::Deserialize;
use task_domain::model::expected_section_data::ExpectedSectionData;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate, Default)]
pub struct SectionDto {
    #[validate(regex(path = "super::UUID_PATTERN", message = "Invalid UUID format"))]
    pub id: String,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    #[validate(url(message = "Invalid URL format"))]
    pub image: Option<String>,
    #[validate]
    pub tasks: Vec<TaskDto>,
}

impl From<SectionDto> for ExpectedSectionData {
    fn from(value: SectionDto) -> Self {
        ExpectedSectionData {
            id: value.id.replace('-', "").to_lowercase(),
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
        let section = SectionDto {
            id: "id-ID".to_string(),
            title: "title".to_string(),
            description: None,
            image: None,
            tasks: vec![task.clone()],
        };
        let expected = ExpectedSectionData {
            id: "idid".to_string(),
            title: "title".to_string(),
            description: None,
            image: None,
            tasks: vec![task.into()],
        };

        let result = ExpectedSectionData::from(section);

        assert_eq!(result, expected);
    }
}
