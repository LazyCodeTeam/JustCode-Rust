use profile_domain::model::user_role::UserRole;
use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProfileRoleDto {
    User,
    Editor,
    Admin,
}

impl From<ProfileRoleDto> for UserRole {
    fn from(dto: ProfileRoleDto) -> Self {
        match dto {
            ProfileRoleDto::User => UserRole::User,
            ProfileRoleDto::Editor => UserRole::Editor,
            ProfileRoleDto::Admin => UserRole::Admin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_profile_role_dto() {
        let dto = ProfileRoleDto::User;
        let role = UserRole::from(dto);
        assert_eq!(role, UserRole::User);
    }

    #[test]
    fn from_profile_role_dto_editor() {
        let dto = ProfileRoleDto::Editor;
        let role = UserRole::from(dto);
        assert_eq!(role, UserRole::Editor);
    }

    #[test]
    fn from_profile_role_dto_admin() {
        let dto = ProfileRoleDto::Admin;
        let role = UserRole::from(dto);
        assert_eq!(role, UserRole::Admin);
    }
}
