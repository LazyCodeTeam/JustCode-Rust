use profile_domain::model::platform::Platform;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlatformDto {
    Android,
    Ios,
}

impl From<Platform> for PlatformDto {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::Android => PlatformDto::Android,
            Platform::Ios => PlatformDto::Ios,
        }
    }
}

impl From<PlatformDto> for Platform {
    fn from(platform: PlatformDto) -> Self {
        match platform {
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
        assert_eq!(PlatformDto::from(Platform::Android), PlatformDto::Android);
        assert_eq!(PlatformDto::from(Platform::Ios), PlatformDto::Ios);
    }

    #[test]
    fn test_platform_from_platform_dto() {
        assert_eq!(Platform::from(PlatformDto::Android), Platform::Android);
        assert_eq!(Platform::from(PlatformDto::Ios), Platform::Ios);
    }
}
