#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct UpdateProfileParams {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
