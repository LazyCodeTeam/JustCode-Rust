use std::collections::HashMap;

use common_domain::error::{Error, ErrorOutput, ErrorType, Result};

pub async fn validate_secret_key(key: Option<&str>, expected_key: Option<&str>) -> Result<()> {
    match (key, expected_key) {
        (_, Some(expected_key)) if expected_key.is_empty() => Err(no_expected_key_error()),
        (Some(key), Some(expected_key)) if key == expected_key => Ok(()),
        (_, None) => Err(no_expected_key_error()),
        _ => Err(invalid_key_error()),
    }
}

fn no_expected_key_error() -> Error {
    Error::unknown("Expected key not provided".to_string())
}

fn invalid_key_error() -> Error {
    Error {
        debug_message: "Invalid secret key".to_string(),
        error_type: ErrorType::Unauthorized,
        output: Box::new(ErrorOutput {
            message: "Unauthorized".to_string(),
            code: "unauthorized".to_string(),
            args: HashMap::new(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn valid_secret_key() {
        let result = validate_secret_key(Some("key"), Some("key"));

        assert!(result.await.is_ok());
    }

    #[tokio::test]
    async fn invalid_secret_key() {
        let result = validate_secret_key(Some("key"), Some("key2")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), invalid_key_error());
    }

    #[tokio::test]
    async fn no_secret_key() {
        let result = validate_secret_key(None, Some("key2")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), invalid_key_error());
    }

    #[tokio::test]
    async fn no_expected_key() {
        let result = validate_secret_key(Some("key"), None).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), no_expected_key_error());
    }

    #[tokio::test]
    async fn empty_expected_key() {
        let result = validate_secret_key(Some("key"), Some("")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), no_expected_key_error());
    }
}
