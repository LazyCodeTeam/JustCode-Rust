use content_domain::model::expected_technology_data::ExpectedTechnologyData;

use crate::{ExpectedTechnologyDto, FromDto, IntoModel};

impl FromDto<ExpectedTechnologyDto> for ExpectedTechnologyData {
    fn from_dto(dto: ExpectedTechnologyDto) -> Self {
        ExpectedTechnologyData {
            id: dto.id.simple().to_string(),
            name: dto.name,
            description: dto.description,
            image: dto.image,
            sections: dto
                .sections
                .into_iter()
                .map(IntoModel::into_model)
                .collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;

    use crate::ExpectedSectionDto;

    use super::*;

    #[test]
    fn test_from() {
        let section = ExpectedSectionDto::default();
        let uuid = Uuid::new_v4();
        let technology = ExpectedTechnologyDto {
            id: uuid,
            name: "title".to_string(),
            description: None,
            image: None,
            sections: vec![section.clone()],
        };
        let expected = ExpectedTechnologyData {
            id: uuid.simple().to_string(),
            name: "title".to_string(),
            description: None,
            image: None,
            sections: vec![section.into_model()],
        };

        let result = ExpectedTechnologyData::from_dto(technology);

        assert_eq!(result, expected);
    }
}
