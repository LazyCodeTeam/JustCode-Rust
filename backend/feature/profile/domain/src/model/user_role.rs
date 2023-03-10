#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy, Default)]
pub enum UserRole {
    #[default]
    User,
    Editor,
    Admin,
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
