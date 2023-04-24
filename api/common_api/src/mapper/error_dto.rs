use crate::{dto::ErrorDto, FromModel};
use common_domain::error::ErrorOutput;

impl FromModel<ErrorOutput> for ErrorDto {
    fn from_model(model: ErrorOutput) -> Self {
        Self {
            message: model.message,
            code: model.code,
            args: model.args,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn from_error_details() {
        let message = "Test message".to_owned();
        let code = "error.test_code".to_owned();
        let args = HashMap::from([("key".to_owned(), "value".to_owned())]);
        let error_details = ErrorOutput {
            message: message.clone(),
            code: code.clone(),
            args: args.clone(),
        };

        assert_eq!(
            ErrorDto::from_model(error_details),
            ErrorDto {
                message,
                code,
                args
            }
        );
    }
}
