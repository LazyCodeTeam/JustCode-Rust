use log::Level;

use crate::boxed::Boxed;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ErrorType {
    InvalidData,
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error {
    pub debug_message: String,
    pub error_type: ErrorType,
    pub details: Box<ErrorDetails>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ErrorDetails {
    pub message: String,
    pub code: String,
    pub args: Option<HashMap<String, String>>,
}

#[derive(Debug)]
pub struct ErrorBuilder {
    debug_message: Option<String>,
    error_type: Option<ErrorType>,
    details: Option<Box<ErrorDetails>>,
}

impl Default for Error {
    fn default() -> Self {
        ErrorBuilder::new().build()
    }
}

impl Error {
    pub fn unknown(message: String) -> Error {
        Error::builder().set_debug_message(message).build()
    }

    pub fn builder() -> ErrorBuilder {
        ErrorBuilder::new()
    }
}

impl From<ErrorType> for Level {
    fn from(ty: ErrorType) -> Self {
        match ty {
            ErrorType::InvalidData => Level::Warn,
            ErrorType::Unknown => Level::Error,
        }
    }
}

impl ErrorBuilder {
    fn new() -> Self {
        Self {
            debug_message: None,
            error_type: None,
            details: None,
        }
    }

    pub fn set_debug_message(mut self, message: String) -> Self {
        self.debug_message = Some(message);
        self
    }

    pub fn set_error_type(mut self, error_type: ErrorType) -> Self {
        self.error_type = Some(error_type);
        self
    }

    pub fn set_details(mut self, details: ErrorDetails) -> Self {
        self.details = Some(details.boxed());
        self
    }

    pub fn build(self) -> Error {
        let debug_message = self.debug_message.unwrap_or_else(|| "".to_owned());
        let error_type = self.error_type.unwrap_or(ErrorType::Unknown);
        let details = self.details.unwrap_or_else(|| {
            ErrorDetails {
                message: "Unknown server error".to_owned(),
                code: "error.unknown".to_owned(),
                args: None,
            }
            .boxed()
        });

        log::log!(Level::from(error_type), "{debug_message:?} - {details:?}");

        Error {
            debug_message,
            error_type,
            details,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn default_value() {
        let value = Error::default();

        assert_eq!(
            value,
            Error {
                debug_message: "".to_owned(),
                error_type: ErrorType::Unknown,
                details: ErrorDetails {
                    message: "Unknown server error".to_owned(),
                    code: "error.unknown".to_owned(),
                    args: None,
                }
                .boxed()
            }
        )
    }

    #[test]
    fn unknown() {
        let value = Error::unknown("Custom message".to_owned());

        assert_eq!(
            value,
            Error {
                debug_message: "Custom message".to_owned(),
                error_type: ErrorType::Unknown,
                details: ErrorDetails {
                    message: "Unknown server error".to_owned(),
                    code: "error.unknown".to_owned(),
                    args: None,
                }
                .boxed()
            }
        )
    }

    #[test]
    fn builder() {
        let debug_message = "Debug message".to_owned();
        let error_type = ErrorType::InvalidData;
        let details = ErrorDetails {
            message: "Error details message".to_owned(),
            code: "error.test_code".to_owned(),
            args: Some(HashMap::from([("test".to_owned(), "arg".to_owned())])),
        };
        let value = Error::builder()
            .set_debug_message(debug_message.to_owned())
            .set_error_type(error_type)
            .set_details(details.clone())
            .build();

        assert_eq!(
            value,
            Error {
                debug_message,
                error_type,
                details: details.boxed(),
            }
        )
    }
}
