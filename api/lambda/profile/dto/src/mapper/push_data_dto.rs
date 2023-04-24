use profile_domain::model::push_data::PushData;

use crate::{FromDto, IntoModel, PushDataDto};

impl FromDto<PushDataDto> for PushData {
    fn from_dto(dto: PushDataDto) -> Self {
        Self {
            token: dto.token,
            platform: dto.platform.into_model(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PlatformDto;
    use profile_domain::model::platform::Platform;

    use super::*;

    #[test]
    fn map_push_data_dto() {
        let dto = PushDataDto {
            token: "token".to_owned(),
            platform: PlatformDto::Android,
        };

        let result = PushData::from_dto(dto);

        assert_eq!(result.token, "token");
        assert_eq!(result.platform, Platform::Android);
    }
}
