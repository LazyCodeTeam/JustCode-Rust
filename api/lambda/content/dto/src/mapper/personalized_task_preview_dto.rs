use content_domain::model::personalized_task_preview::PersonalizedTaskPreview;
use gen::models::PersonalizedTaskPreviewDto;

use crate::MapFrom;

impl MapFrom<PersonalizedTaskPreview> for PersonalizedTaskPreviewDto {
    fn map_from(value: PersonalizedTaskPreview) -> Self {
        Self {
            id: value.id,
            title: value.title,
            done_at: value.done_at.map(|d| d.to_rfc3339()),
        }
    }
}
