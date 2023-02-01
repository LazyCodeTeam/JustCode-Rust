use profile_domain::model::push_data::PushData;
use serde::Deserialize;
use validator::Validate;

use super::platform_dto::PlatformDto;

#[derive(Deserialize, Validate, Clone, PartialEq, Eq, Debug)]
pub struct PushDataDto {
    #[validate(length(min = 1))]
    pub token: String,
    pub platform: PlatformDto,
}

impl From<PushDataDto> for PushData {
    fn from(value: PushDataDto) -> Self {
        Self {
            token: value.token,
            platform: value.platform.into(),
        }
    }
}

#[cfg(test)]
mod test {
    use profile_domain::model::platform::Platform;

    use super::*;

    #[test]
    fn map_push_data_dto() {
        let dto = PushDataDto {
            token: "token".to_owned(),
            platform: PlatformDto::Android,
        };

        let result = PushData::from(dto);

        assert_eq!(result.token, "token");
        assert_eq!(result.platform, Platform::Android);
    }
}
