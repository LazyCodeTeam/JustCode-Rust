use profile_domain::model::profile::Profile;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl From<Profile> for ProfileDto {
    fn from(profile: Profile) -> Self {
        Self {
            id: profile.id,
            name: profile.name,
            email: profile.email,
            avatar_url: profile.avatar_url,
            first_name: profile.first_name,
            last_name: profile.last_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_profile() {
        let profile = Profile {
            id: "id".to_string(),
            name: "John Doe".to_owned(),
            email: "test@email.com".to_owned(),
            avatar_url: Some("https://example.com/avatar.png".to_owned()),
            first_name: Some("John".to_owned()),
            last_name: Some("Doe".to_owned()),
            ..Default::default()
        };
        let profile_dto = ProfileDto::from(profile);
        assert_eq!(
            profile_dto,
            ProfileDto {
                id: "id".to_string(),
                name: "John Doe".to_owned(),
                email: "test@email.com".to_owned(),
                avatar_url: Some("https://example.com/avatar.png".to_owned()),
                first_name: Some("John".to_owned()),
                last_name: Some("Doe".to_owned()),
            }
        );
    }
}
