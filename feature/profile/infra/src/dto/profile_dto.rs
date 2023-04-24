use chrono::{DateTime, Utc};
use profile_domain::model::{create_profile_params::CreateProfileParams, profile::Profile};
use serde::{Deserialize, Serialize};

use crate::{FromDto, FromModel, IntoDto, IntoModel, PROFILE_ID_PREFIX, PROFILE_PRIMARY_KEY};

use super::profile_role_dto::ProfileRoleDto;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    #[serde(rename = "SK")]
    pub id: String,
    #[serde(rename = "PK")]
    pub pk: String,
    pub email: String,
    pub name: String,
    pub role: Option<ProfileRoleDto>,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl FromDto<ProfileDto> for Profile {
    fn from_dto(dto: ProfileDto) -> Self {
        Profile {
            id: dto.id.replace(PROFILE_ID_PREFIX, ""),
            name: dto.name,
            email: dto.email,
            avatar_url: dto.avatar_url,
            first_name: dto.first_name,
            last_name: dto.last_name,
            role: dto.role.map(IntoModel::into_model).unwrap_or_default(),
            created_at: dto.created_at,
        }
    }
}

impl FromModel<CreateProfileParams> for ProfileDto {
    fn from_model(model: CreateProfileParams) -> Self {
        ProfileDto {
            id: format!("{}{}", PROFILE_ID_PREFIX, model.id),
            pk: PROFILE_PRIMARY_KEY.to_string(),
            name: model.name,
            email: model.email,
            avatar_url: None,
            first_name: None,
            last_name: None,
            role: Some(ProfileRoleDto::User),
            created_at: Utc::now(),
        }
    }
}

impl FromModel<Profile> for ProfileDto {
    fn from_model(model: Profile) -> Self {
        ProfileDto {
            id: format!("{}{}", PROFILE_ID_PREFIX, model.id),
            pk: PROFILE_PRIMARY_KEY.to_string(),
            name: model.name,
            email: model.email,
            avatar_url: model.avatar_url,
            first_name: model.first_name,
            last_name: model.last_name,
            role: Some(model.role.into_dto()),
            created_at: model.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use profile_domain::model::user_role::UserRole;

    use super::*;

    #[test]
    fn from_create_profile_params() {
        let params = CreateProfileParams {
            id: "id".to_string(),
            name: "name".to_string(),
            email: "email".to_string(),
        };

        let result = ProfileDto::from_model(params);

        assert_eq!(result.id, format!("{PROFILE_ID_PREFIX}id"));
        assert_eq!(result.name, "name");
        assert_eq!(result.email, "email");
        assert_eq!(result.avatar_url, None);
        assert_eq!(result.first_name, None);
        assert_eq!(result.last_name, None);
        assert_eq!(result.role, Some(ProfileRoleDto::User));
        assert!(result.created_at <= Utc::now());
    }

    #[test]
    fn from_profile_dto() {
        let now = Utc::now();
        let dto = ProfileDto {
            id: format!("{PROFILE_ID_PREFIX}id"),
            pk: PROFILE_PRIMARY_KEY.to_string(),
            name: "name".to_string(),
            email: "email".to_string(),
            avatar_url: Some("avatar_url".to_string()),
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
            role: Some(ProfileRoleDto::Admin),
            created_at: now,
        };

        assert_eq!(
            Profile::from_dto(dto),
            Profile {
                id: "id".to_string(),
                name: "name".to_string(),
                email: "email".to_string(),
                avatar_url: Some("avatar_url".to_string()),
                first_name: Some("first_name".to_string()),
                last_name: Some("last_name".to_string()),
                role: UserRole::Admin,
                created_at: now,
            },
        );
    }

    #[test]
    fn from_profile() {
        let now = Utc::now();
        let profile = Profile {
            id: "id".to_string(),
            name: "name".to_string(),
            email: "email".to_string(),
            avatar_url: Some("avatar_url".to_string()),
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
            role: UserRole::Admin,
            created_at: now,
        };

        assert_eq!(
            ProfileDto::from_model(profile),
            ProfileDto {
                id: format!("{PROFILE_ID_PREFIX}id"),
                pk: PROFILE_PRIMARY_KEY.to_string(),
                name: "name".to_string(),
                email: "email".to_string(),
                avatar_url: Some("avatar_url".to_string()),
                first_name: Some("first_name".to_string()),
                last_name: Some("last_name".to_string()),
                role: Some(ProfileRoleDto::Admin),
                created_at: now,
            },
        );
    }
}
