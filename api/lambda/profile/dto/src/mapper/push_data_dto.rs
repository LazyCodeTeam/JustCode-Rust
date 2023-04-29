use profile_domain::model::push_data::PushData;

use crate::{MapFrom, MapInto, PushDataDto};

impl MapFrom<PushDataDto> for PushData {
    fn map_from(dto: PushDataDto) -> Self {
        Self {
            token: dto.token,
            platform: dto.platform.map_into(),
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

        let result = PushData::map_from(dto);

        assert_eq!(result.token, "token");
        assert_eq!(result.platform, Platform::Android);
    }
}
