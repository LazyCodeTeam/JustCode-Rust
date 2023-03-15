use profile_domain::model::profile::Profile;

use crate::{FromModel, ProfileDto};

impl FromModel<Profile> for ProfileDto {
    fn from_model(model: Profile) -> Self {
        Self {
            id: model.id,
            name: model.name,
            email: model.email,
            avatar_url: model.avatar_url,
            first_name: model.first_name,
            last_name: model.last_name,
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
        let profile_dto = ProfileDto::from_model(profile);
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
