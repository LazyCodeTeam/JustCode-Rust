use content_domain::model::expected_technology_data::ExpectedTechnologyData;

use crate::{ExpectedTechnologyDto, MapFrom, MapInto};

impl MapFrom<ExpectedTechnologyDto> for ExpectedTechnologyData {
    fn map_from(dto: ExpectedTechnologyDto) -> Self {
        ExpectedTechnologyData {
            id: dto.id.simple().to_string(),
            name: dto.name,
            description: dto.description,
            image: dto.image,
            sections: dto.sections.into_iter().map(MapInto::map_into).collect(),
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
            sections: vec![section.map_into()],
        };

        let result = ExpectedTechnologyData::map_from(technology);

        assert_eq!(result, expected);
    }
}
