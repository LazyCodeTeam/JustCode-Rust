use content_domain::model::personalized_section::PersonalizedSection;
use gen::models::PersonalizedSectionDto;

use crate::{MapFrom, MapInto};

impl MapFrom<PersonalizedSection> for PersonalizedSectionDto {
    fn map_from(value: PersonalizedSection) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            image: value.image,
            tasks_preview: value.tasks_preview.map_into(),
        }
    }
}
