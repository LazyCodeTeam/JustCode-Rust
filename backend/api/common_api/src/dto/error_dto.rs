use common_domain::error::ErrorDetails;
use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct ErrorDto {
    pub message: String,
    pub code: String,
    pub args: Option<HashMap<String, String>>,
}

impl From<ErrorDetails> for ErrorDto {
    fn from(error: ErrorDetails) -> Self {
        Self {
            message: error.message,
            code: error.code,
            args: error.args,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from() {
        let message = "Test message".to_owned();
        let code = "error.test_code".to_owned();
        let args = Some(HashMap::from([("key".to_owned(), "value".to_owned())]));
        let error_details = ErrorDetails {
            message: message.clone(),
            code: code.clone(),
            args: args.clone(),
        };

        assert_eq!(
            ErrorDto::from(error_details),
            ErrorDto {
                message,
                code,
                args
            }
        );
    }
}
