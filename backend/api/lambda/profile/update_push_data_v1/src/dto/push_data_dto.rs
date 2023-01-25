use lazy_static::lazy_static;
use profile_domain::model::{platform::Platform, push_data::PushData};
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

lazy_static! {
    static ref PLATFORM_REGEX: Regex = Regex::new(r"^(ANDROID)|(IOS)$").unwrap();
}

#[derive(Deserialize, Validate, Clone, PartialEq, Eq, Debug)]
pub struct PushDataDto {
    #[validate(length(min = 1))]
    pub token: String,
    #[validate(regex(
        path = "PLATFORM_REGEX",
        message = "Platform must be either ANDROID or IOS"
    ))]
    pub platform: String,
}

impl From<PushDataDto> for PushData {
    fn from(value: PushDataDto) -> Self {
        Self {
            token: value.token,
            platform: match value.platform.as_str() {
                "ANDROID" => Platform::Android,
                "IOS" => Platform::Ios,
                _ => Platform::Unknown,
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn map_android_push_data() {
        let dto = PushDataDto {
            token: "token".to_owned(),
            platform: "ANDROID".to_owned(),
        };

        let result = PushData::from(dto);

        assert_eq!(result.token, "token");
        assert_eq!(result.platform, Platform::Android);
    }

    #[test]
    fn map_ios_push_data() {
        let dto = PushDataDto {
            token: "token".to_owned(),
            platform: "IOS".to_owned(),
        };

        let result = PushData::from(dto);

        assert_eq!(result.token, "token");
        assert_eq!(result.platform, Platform::Ios);
    }

    #[test]
    fn map_unknown_push_data() {
        let dto = PushDataDto {
            token: "token".to_owned(),
            platform: "something".to_owned(),
        };

        let result = PushData::from(dto);

        assert_eq!(result.token, "token");
        assert_eq!(result.platform, Platform::Unknown);
    }
}
