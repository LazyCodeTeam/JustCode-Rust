use super::section_dto::SectionDto;
use serde::Deserialize;
use task_domain::model::expected_technology_data::ExpectedTechnologyData;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate, Default)]
pub struct TechnologyDto {
    #[validate(regex(path = "super::UUID_PATTERN", message = "Invalid UUID format"))]
    pub id: String,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    #[validate(url(message = "Invalid URL format"))]
    pub image: Option<String>,
    #[validate]
    pub sections: Vec<SectionDto>,
}

impl From<TechnologyDto> for ExpectedTechnologyData {
    fn from(value: TechnologyDto) -> Self {
        ExpectedTechnologyData {
            id: value.id.replace('-', "").to_lowercase(),
            title: value.title,
            description: value.description,
            image: value.image,
            sections: value.sections.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let section = SectionDto::default();
        let technology = TechnologyDto {
            id: "id-ID".to_string(),
            title: "title".to_string(),
            description: None,
            image: None,
            sections: vec![section.clone()],
        };
        let expected = ExpectedTechnologyData {
            id: "idid".to_string(),
            title: "title".to_string(),
            description: None,
            image: None,
            sections: vec![section.into()],
        };

        let result = ExpectedTechnologyData::from(technology);

        assert_eq!(result, expected);
    }
}
