use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct VersionResponseDto {
    pub version: String,
}

impl From<String> for VersionResponseDto {
    fn from(s: String) -> Self {
        Self { version: s }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_string() {
        let version = "version".to_owned();

        let dto: VersionResponseDto = version.into();

        assert_eq!(
            dto,
            VersionResponseDto {
                version: "version".to_owned()
            }
        )
    }
}
