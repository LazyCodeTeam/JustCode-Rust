use chrono::{DateTime, Utc};

use super::{update_profile_params::UpdateProfileParams, user_role::UserRole};

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Profile {
    pub fn update(self, params: UpdateProfileParams) -> Self {
        Self {
            id: self.id,
            name: self.name,
            email: self.email,
            avatar_url: self.avatar_url,
            role: self.role,
            first_name: params.first_name,
            last_name: params.last_name,
            created_at: self.created_at,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_update() {
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
        let params = UpdateProfileParams {
            first_name: Some("first_name2".to_string()),
            last_name: Some("last_name2".to_string()),
        };

        let updated_profile = profile.update(params);

        assert_eq!(updated_profile.id, "id");
        assert_eq!(updated_profile.name, "name");
        assert_eq!(updated_profile.email, "email");
        assert_eq!(updated_profile.avatar_url, Some("avatar_url".to_string()));
        assert_eq!(updated_profile.first_name, Some("first_name2".to_string()));
        assert_eq!(updated_profile.last_name, Some("last_name2".to_string()));
        assert_eq!(updated_profile.role, UserRole::Admin);
        assert_eq!(updated_profile.created_at, now);
    }
}
