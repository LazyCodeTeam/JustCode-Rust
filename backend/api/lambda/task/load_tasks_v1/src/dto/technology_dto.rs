use super::section_dto::SectionDto;
#[cfg(feature = "fake_dto")]
use fake::{Dummy, Fake, Faker};
use serde::Deserialize;
use task_domain::model::expected_technology_data::ExpectedTechnologyData;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Validate, Default)]
#[cfg_attr(feature = "fake_dto", derive(Dummy, serde::Serialize))]
pub struct TechnologyDto {
    pub id: Uuid,
    #[validate(length(min = 1))]
    pub title: String,
    pub description: Option<String>,
    #[validate(url(message = "Invalid URL format"))]
    pub image: Option<String>,
    #[validate]
    #[dummy(faker = "(Faker, 3..10)")]
    pub sections: Vec<SectionDto>,
}

impl From<TechnologyDto> for ExpectedTechnologyData {
    fn from(value: TechnologyDto) -> Self {
        ExpectedTechnologyData {
            id: value.id.simple().to_string(),
            name: value.title,
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
        let uuid = Uuid::new_v4();
        let technology = TechnologyDto {
            id: uuid,
            title: "title".to_string(),
            description: None,
            image: None,
            sections: vec![section.clone()],
        };
        let expected = ExpectedTechnologyData {
            id: uuid.simple().to_string(),
            name: "title".to_string(),
            description: None,
            image: None,
            sections: vec![section.into()],
        };

        let result = ExpectedTechnologyData::from(technology);

        assert_eq!(result, expected);
    }
}
