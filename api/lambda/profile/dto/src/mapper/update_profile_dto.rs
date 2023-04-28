use profile_domain::model::update_profile_params::UpdateProfileParams;

use crate::{MapFrom, UpdateProfileDto};

impl MapFrom<UpdateProfileDto> for UpdateProfileParams {
    fn map_from(dto: UpdateProfileDto) -> Self {
        Self {
            first_name: dto.first_name,
            last_name: dto.last_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_update_profile_dto() {
        let dto = UpdateProfileDto {
            first_name: Some("John".to_owned()),
            last_name: Some("Doe".to_owned()),
        };
        let params = UpdateProfileParams::map_from(dto);
        assert_eq!(
            params,
            UpdateProfileParams {
                first_name: Some("John".to_owned()),
                last_name: Some("Doe".to_owned()),
            }
        );
    }
}
