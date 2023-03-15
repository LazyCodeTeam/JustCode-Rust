use content_domain::model::expected_section_data::ExpectedSectionData;

use crate::{ExpectedSectionDto, FromDto, IntoModel};

impl FromDto<ExpectedSectionDto> for ExpectedSectionData {
    fn from_dto(dto: ExpectedSectionDto) -> Self {
        ExpectedSectionData {
            id: dto.id.simple().to_string(),
            title: dto.title,
            description: dto.description,
            image: dto.image,
            tasks: dto.tasks.into_iter().map(IntoModel::into_model).collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use gen::models::ExpectedTaskDto;
    use uuid::Uuid;

    use super::*;

    #[test]
    fn test_from() {
        let task = ExpectedTaskDto::default();
        let uuid = Uuid::new_v4();
        let section = ExpectedSectionDto {
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
            tasks: vec![task.into_model()],
        };

        let result = ExpectedSectionData::from_dto(section);

        assert_eq!(result, expected);
    }
}
