#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GitHookEvent {
    Push { reference: String },
    Unknown,
}
