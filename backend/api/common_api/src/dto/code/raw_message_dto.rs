use code_domain::model::raw_message::RawMessage;
use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct RawMessageDto {
    pub success: bool,
    pub message: String,
}

impl From<RawMessage> for RawMessageDto {
    fn from(raw_message: RawMessage) -> Self {
        RawMessageDto {
            message: raw_message.message,
            success: raw_message.success,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_string() {
        let raw_message = RawMessage {
            message: "message".to_owned(),
            success: true,
        };
        assert_eq!(
            RawMessageDto::from(raw_message),
            RawMessageDto {
                success: true,
                message: "message".to_owned()
            }
        )
    }
}
