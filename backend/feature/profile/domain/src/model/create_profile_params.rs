#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct CreateProfileParams {
    pub id: String,
    pub name: String,
    pub email: String,
}
