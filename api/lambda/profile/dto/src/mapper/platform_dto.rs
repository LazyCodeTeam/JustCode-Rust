use profile_domain::model::platform::Platform;

use crate::{FromDto, PlatformDto};

impl FromDto<PlatformDto> for Platform {
    fn from_dto(dto: PlatformDto) -> Self {
        match dto {
            PlatformDto::Android => Platform::Android,
            PlatformDto::Ios => Platform::Ios,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_android_push_data() {
        let dto = PlatformDto::Android;

        let result = Platform::from_dto(dto);

        assert_eq!(result, Platform::Android);
    }

    #[test]
    fn map_ios_push_data() {
        let dto = PlatformDto::Ios;

        let result = Platform::from_dto(dto);

        assert_eq!(result, Platform::Ios);
    }
}
