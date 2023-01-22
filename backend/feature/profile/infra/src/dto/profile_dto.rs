use chrono::{DateTime, Utc};
use profile_domain::model::{profile::Profile, user_role::UserRole};
use serde::Deserialize;

use crate::PROFILE_ID_PREFIX;

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct ProfileDto {
    #[serde(rename = "PK")]
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: Option<String>,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub updated_at: DateTime<Utc>,
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
            role: dto
                .role
                .map(|role| map_role(role.as_str()))
                .unwrap_or_default(),
            updated_at: dto.updated_at,
            created_at: dto.created_at,
        }
    }
}

fn map_role(role: &str) -> UserRole {
    match role {
        "USER" => UserRole::User,
        "EDITOR" => UserRole::Editor,
        "ADMIN" => UserRole::Admin,
        _ => UserRole::User,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_profile_dto() {
        let now = Utc::now();
        let dto = ProfileDto {
            id: format!("{}id", PROFILE_ID_PREFIX),
            name: "name".to_string(),
            email: "email".to_string(),
            avatar_url: Some("avatar_url".to_string()),
            first_name: Some("first_name".to_string()),
            last_name: Some("last_name".to_string()),
            role: Some("ADMIN".to_string()),
            updated_at: now,
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
                updated_at: now,
                created_at: now,
            },
        );
    }

    #[test]
    fn map_admin_role() {
        assert_eq!(map_role("ADMIN"), UserRole::Admin);
    }

    #[test]
    fn map_editor_role() {
        assert_eq!(map_role("EDITOR"), UserRole::Editor);
    }

    #[test]
    fn map_user_role() {
        assert_eq!(map_role("USER"), UserRole::User);
    }

    #[test]
    fn map_unknown_role() {
        assert_eq!(map_role("UNKNOWN"), UserRole::User);
    }
}
