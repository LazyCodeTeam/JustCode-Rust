use profile_domain::model::platform::Platform;

use crate::{MapFrom, PlatformDto};

impl MapFrom<PlatformDto> for Platform {
    fn map_from(dto: PlatformDto) -> Self {
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

        let result = Platform::map_from(dto);

        assert_eq!(result, Platform::Android);
    }

    #[test]
    fn map_ios_push_data() {
        let dto = PlatformDto::Ios;

        let result = Platform::map_from(dto);

        assert_eq!(result, Platform::Ios);
    }
}
