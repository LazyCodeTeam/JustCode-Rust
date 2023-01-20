#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}
