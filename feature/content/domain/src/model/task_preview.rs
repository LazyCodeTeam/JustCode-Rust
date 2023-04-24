#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct TaskPreview {
    pub id: String,
    pub title: String,
    pub for_anonymous: bool,
}
