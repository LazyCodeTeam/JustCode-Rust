use profile_domain::model::update_profile_params::UpdateProfileParams;
use serde::Deserialize;

#[derive(Deserialize, Clone, Default, PartialEq, Eq, Debug)]
pub struct UpdateProfileDto {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<UpdateProfileDto> for UpdateProfileParams {
    fn from(dto: UpdateProfileDto) -> Self {
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
        let params = UpdateProfileParams::from(dto);
        assert_eq!(
            params,
            UpdateProfileParams {
                first_name: Some("John".to_owned()),
                last_name: Some("Doe".to_owned()),
            }
        );
    }
}
