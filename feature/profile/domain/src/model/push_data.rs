use super::platform::Platform;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PushData {
    pub token: String,
    pub platform: Platform,
}
