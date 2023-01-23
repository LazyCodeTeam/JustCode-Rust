use chrono::{DateTime, Utc};
use profile_domain::model::{create_profile_params::CreateProfileParams, profile::Profile};
use serde::Serialize;

use crate::{PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

#[derive(Serialize, Default, PartialEq, Eq, Debug)]
pub struct PutProfileDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub name: String,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<CreateProfileParams> for PutProfileDto {
    fn from(params: CreateProfileParams) -> Self {
        Self {
            pk: format!("{}{}", PROFILE_ID_PREFIX, params.id),
            sk: PROFILE_SORT_KEY.to_owned(),
            name: params.name,
            email: params.email,
            updated_at: Utc::now(),
            created_at: Utc::now(),
            ..Default::default()
        }
    }
}

impl From<Profile> for PutProfileDto {
    fn from(profile: Profile) -> Self {
        Self {
            pk: format!("{}{}", PROFILE_ID_PREFIX, profile.id),
            sk: PROFILE_SORT_KEY.to_owned(),
            name: profile.name,
            email: profile.email,
            first_name: profile.first_name,
            last_name: profile.last_name,
            updated_at: profile.updated_at,
            created_at: profile.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_create_params() {
        let params = CreateProfileParams {
            id: "id".to_owned(),
            name: "name".to_owned(),
            email: "email".to_owned(),
        };
        let dto = PutProfileDto::from(params);
        assert_eq!(
            dto,
            PutProfileDto {
                pk: format!("{}{}", PROFILE_ID_PREFIX, "id"),
                sk: PROFILE_SORT_KEY.to_owned(),
                name: "name".to_owned(),
                email: "email".to_owned(),
                updated_at: dto.updated_at,
                created_at: dto.created_at,
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_from_profile() {
        let now = Utc::now();
        let profile = Profile {
            id: "id".to_string(),
            name: "John Doe".to_owned(),
            email: "email".to_owned(),
            avatar_url: Some("https://example.com/avatar.png".to_owned()),
            first_name: Some("John".to_owned()),
            last_name: Some("Doe".to_owned()),
            role: Default::default(),
            updated_at: now,
            created_at: now,
        };

        let dto = PutProfileDto::from(profile);

        assert_eq!(
            dto,
            PutProfileDto {
                pk: format!("{}{}", PROFILE_ID_PREFIX, "id"),
                sk: PROFILE_SORT_KEY.to_owned(),
                name: "John Doe".to_owned(),
                email: "email".to_owned(),
                first_name: Some("John".to_owned()),
                last_name: Some("Doe".to_owned()),
                updated_at: now,
                created_at: now,
            }
        );
    }
}
