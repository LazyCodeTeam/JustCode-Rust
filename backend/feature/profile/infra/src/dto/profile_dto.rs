use chrono::{DateTime, Utc};
use profile_domain::model::profile::Profile;
use serde::Deserialize;

use crate::PROFILE_ID_PREFIX;

use super::profile_role_dto::ProfileRoleDto;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    #[serde(rename = "PK")]
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: Option<ProfileRoleDto>,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<ProfileDto> for Profile {
    fn from(dto: ProfileDto) -> Self {
        Profile {
            id: dto.id.replace(PROFILE_ID_PREFIX, ""),
            name: dto.name,
            email: dto.email,
            avatar_url: dto.avatar_url,
            first_name: dto.first_name,
            last_name: dto.last_name,
            role: dto.role.map(|r| r.into()).unwrap_or_default(),
            created_at: dto.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use profile_domain::model::user_role::UserRole;

    use super::*;

    #[test]
    fn from_profile_dto() {
        let now = Utc::now();
        let dto = ProfileDto {
            id: format!("{PROFILE_ID_PREFIX}id"),
            name: "name".to_string(),
            email: "email".to_string(),
            avatar_url: Some("avatar_url".to_string()),
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
            role: Some(ProfileRoleDto::Admin),
            created_at: now,
        };

        assert_eq!(
            Profile::from(dto),
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
}
