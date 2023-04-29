use profile_domain::model::platform::Platform;
use serde::{Deserialize, Serialize};

use crate::MapFrom;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlatformDto {
    Android,
    Ios,
}

impl MapFrom<Platform> for PlatformDto {
    fn map_from(model: Platform) -> Self {
        match model {
            Platform::Android => PlatformDto::Android,
            Platform::Ios => PlatformDto::Ios,
        }
    }
}

impl MapFrom<PlatformDto> for Platform {
    fn map_from(dto: PlatformDto) -> Self {
        match dto {
            PlatformDto::Android => Platform::Android,
            PlatformDto::Ios => Platform::Ios,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_dto_from_platform() {
        assert_eq!(
            PlatformDto::map_from(Platform::Android),
            PlatformDto::Android
        );
        assert_eq!(PlatformDto::map_from(Platform::Ios), PlatformDto::Ios);
    }

    #[test]
    fn test_platform_from_platform_dto() {
        assert_eq!(Platform::map_from(PlatformDto::Android), Platform::Android);
        assert_eq!(Platform::map_from(PlatformDto::Ios), Platform::Ios);
    }
}
