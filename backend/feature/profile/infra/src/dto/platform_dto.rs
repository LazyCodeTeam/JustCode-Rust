use profile_domain::model::platform::Platform;
use serde::{Deserialize, Serialize};

use crate::{FromDto, FromModel};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlatformDto {
    Android,
    Ios,
}

impl FromModel<Platform> for PlatformDto {
    fn from_model(model: Platform) -> Self {
        match model {
            Platform::Android => PlatformDto::Android,
            Platform::Ios => PlatformDto::Ios,
        }
    }
}

impl FromDto<PlatformDto> for Platform {
    fn from_dto(dto: PlatformDto) -> Self {
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
            PlatformDto::from_model(Platform::Android),
            PlatformDto::Android
        );
        assert_eq!(PlatformDto::from_model(Platform::Ios), PlatformDto::Ios);
    }

    #[test]
    fn test_platform_from_platform_dto() {
        assert_eq!(Platform::from_dto(PlatformDto::Android), Platform::Android);
        assert_eq!(Platform::from_dto(PlatformDto::Ios), Platform::Ios);
    }
}
