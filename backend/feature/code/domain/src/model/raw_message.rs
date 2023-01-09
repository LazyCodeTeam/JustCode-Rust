#[derive(PartialEq, Eq, Clone, Debug)]
pub struct RawMessage {
    pub success: bool,
    pub message: String,
}
