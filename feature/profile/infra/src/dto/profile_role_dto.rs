use profile_domain::model::user_role::UserRole;
use serde::{Deserialize, Serialize};

use crate::MapFrom;

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProfileRoleDto {
    User,
    Editor,
    Admin,
}

impl MapFrom<ProfileRoleDto> for UserRole {
    fn map_from(dto: ProfileRoleDto) -> Self {
        match dto {
            ProfileRoleDto::User => UserRole::User,
            ProfileRoleDto::Editor => UserRole::Editor,
            ProfileRoleDto::Admin => UserRole::Admin,
        }
    }
}

impl MapFrom<UserRole> for ProfileRoleDto {
    fn map_from(model: UserRole) -> Self {
        match model {
            UserRole::User => ProfileRoleDto::User,
            UserRole::Editor => ProfileRoleDto::Editor,
            UserRole::Admin => ProfileRoleDto::Admin,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_profile_role_dto() {
        let dto = ProfileRoleDto::User;
        let role = UserRole::map_from(dto);
        assert_eq!(role, UserRole::User);
    }

    #[test]
    fn from_profile_role_dto_editor() {
        let dto = ProfileRoleDto::Editor;
        let role = UserRole::map_from(dto);
        assert_eq!(role, UserRole::Editor);
    }

    #[test]
    fn from_profile_role_dto_admin() {
        let dto = ProfileRoleDto::Admin;
        let role = UserRole::map_from(dto);
        assert_eq!(role, UserRole::Admin);
    }

    #[test]
    fn from_user_role() {
        let role = UserRole::User;
        let dto = ProfileRoleDto::map_from(role);
        assert_eq!(dto, ProfileRoleDto::User);
    }

    #[test]
    fn from_user_role_editor() {
        let role = UserRole::Editor;
        let dto = ProfileRoleDto::map_from(role);
        assert_eq!(dto, ProfileRoleDto::Editor);
    }

    #[test]
    fn from_user_role_admin() {
        let role = UserRole::Admin;
        let dto = ProfileRoleDto::map_from(role);
        assert_eq!(dto, ProfileRoleDto::Admin);
    }
}
