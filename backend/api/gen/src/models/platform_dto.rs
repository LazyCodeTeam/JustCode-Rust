/*
 * just-code-dev
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023-03-10 06:01:08UTC
 *
 * Generated by: https://openapi-generator.tech
 */

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformDto {
    #[serde(rename = "IOS")]
    Ios,
    #[serde(rename = "ANDROID")]
    Android,
}

impl ToString for PlatformDto {
    fn to_string(&self) -> String {
        match self {
            Self::Ios => String::from("IOS"),
            Self::Android => String::from("ANDROID"),
        }
    }
}

impl Default for PlatformDto {
    fn default() -> PlatformDto {
        Self::Ios
    }
}
