#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub enum UserRole {
    User,
    Editor,
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::User
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_role_default() {
        assert_eq!(UserRole::default(), UserRole::User);
    }

    #[test]
    fn user_role_ord() {
        assert!(UserRole::User < UserRole::Editor);
        assert!(UserRole::Editor < UserRole::Admin);
    }
}