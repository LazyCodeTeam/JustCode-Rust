use log::Level;

use std::{collections::HashMap, fmt::Display};

pub type Result<T> = std::result::Result<T, Error>;

const UNKNOWN_ERROR_MESSAGE: &str = "Unknown server error";
const UNKNOWN_ERROR_CODE: &str = "unknown";

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum ErrorType {
    InvalidInput,
    Conflict,
    NotFound,
    NotModified,
    Forbidden,
    Unauthorized,
    #[default]
    Unknown,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Error {
    pub debug_message: String,
    pub error_type: ErrorType,
    pub output: Box<ErrorOutput>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ErrorOutput {
    pub message: String,
    pub code: String,
    pub args: HashMap<String, String>,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            debug_message: "".to_owned(),
            error_type: ErrorType::Unknown,
            output: Default::default(),
        }
    }
}

impl Default for ErrorOutput {
    fn default() -> Self {
        Self {
            message: UNKNOWN_ERROR_MESSAGE.to_owned(),
            code: UNKNOWN_ERROR_CODE.to_owned(),
            args: HashMap::new(),
        }
    }
}

impl Error {
    pub fn unknown(message: impl ToString) -> Self {
        Self {
            debug_message: message.to_string(),
            ..Default::default()
        }
    }

    pub fn not_found() -> Self {
        Self {
            error_type: ErrorType::NotFound,
            output: Box::new(ErrorOutput {
                message: "Not found".to_owned(),
                code: "not_found".to_owned(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    pub fn log(&self) {
        log::log!(self.error_type.into(), "{self}");
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<ErrorType> for Level {
    fn from(ty: ErrorType) -> Self {
        match ty {
            ErrorType::InvalidInput => Level::Info,
            ErrorType::Unknown => Level::Error,
            ErrorType::Conflict => Level::Info,
            ErrorType::Forbidden => Level::Info,
            ErrorType::NotModified => Level::Debug,
            ErrorType::Unauthorized => Level::Info,
            ErrorType::NotFound => Level::Info,
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
                output: Box::new(ErrorOutput {
                    message: UNKNOWN_ERROR_MESSAGE.to_owned(),
                    code: UNKNOWN_ERROR_CODE.to_owned(),
                    args: HashMap::new(),
                })
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
                output: Box::new(ErrorOutput {
                    message: UNKNOWN_ERROR_MESSAGE.to_owned(),
                    code: UNKNOWN_ERROR_CODE.to_owned(),
                    args: HashMap::new(),
                })
            }
        )
    }

    #[test]
    fn default_output() {
        let out = ErrorOutput::default();

        assert_eq!(
            out,
            ErrorOutput {
                message: UNKNOWN_ERROR_MESSAGE.to_owned(),
                code: UNKNOWN_ERROR_CODE.to_owned(),
                args: HashMap::new(),
            }
        )
    }

    #[test]
    fn default_error_type() {
        let ty = ErrorType::default();

        assert_eq!(ty, ErrorType::Unknown)
    }

    #[test]
    fn error_type_from_level() {
        assert_eq!(Level::from(ErrorType::InvalidInput), Level::Info);
        assert_eq!(Level::from(ErrorType::Unknown), Level::Error);
        assert_eq!(Level::from(ErrorType::Conflict), Level::Info);
        assert_eq!(Level::from(ErrorType::Forbidden), Level::Info);
        assert_eq!(Level::from(ErrorType::Unauthorized), Level::Info);
        assert_eq!(Level::from(ErrorType::NotFound), Level::Info);
    }

    #[test]
    fn not_found() {
        let value = Error::not_found();

        assert_eq!(
            value,
            Error {
                error_type: ErrorType::NotFound,
                output: Box::new(ErrorOutput {
                    message: "Not found".to_owned(),
                    code: "not_found".to_owned(),
                    ..Default::default()
                }),
                ..Default::default()
            }
        )
    }
}