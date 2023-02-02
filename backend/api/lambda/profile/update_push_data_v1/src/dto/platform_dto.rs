use profile_domain::model::platform::Platform;
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum PlatformDto {
    Android,
    Ios,
}

impl From<PlatformDto> for Platform {
    fn from(value: PlatformDto) -> Self {
        match value {
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

        let result = Platform::from(dto);

        assert_eq!(result, Platform::Android);
    }

    #[test]
    fn map_ios_push_data() {
        let dto = PlatformDto::Ios;

        let result = Platform::from(dto);

        assert_eq!(result, Platform::Ios);
    }
}
