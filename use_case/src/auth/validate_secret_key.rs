use snafu::Snafu;

pub async fn validate_secret_key(
    key: Option<&str>,
    expected_key: Option<&str>,
) -> Result<(), ValidateSecretKeyError> {
    match (key, expected_key) {
        (_, None) => Err(ValidateSecretKeyError::NoExpectedKey),
        (_, Some(expected_key)) if expected_key.is_empty() => {
            Err(ValidateSecretKeyError::NoExpectedKey)
        }
        (None, _) => Err(ValidateSecretKeyError::NoKey),
        (Some(key), _) if key.is_empty() => Err(ValidateSecretKeyError::NoKey),
        (Some(key), Some(expected_key)) if key != expected_key => {
            Err(ValidateSecretKeyError::InvalidKey)
        }
        _ => Ok(()),
    }
}

#[derive(Debug, PartialEq, Snafu)]
pub enum ValidateSecretKeyError {
    #[snafu(display("Invalid secret key"))]
    InvalidKey,
    #[snafu(display("Expected key not provided"))]
    NoExpectedKey,
    #[snafu(display("No secret key provided"))]
    NoKey,
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn valid_secret_key() {
        let result = validate_secret_key(Some("key"), Some("key")).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn invalid_secret_key() {
        let result = validate_secret_key(Some("key"), Some("key2")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidateSecretKeyError::InvalidKey);
    }

    #[tokio::test]
    async fn no_secret_key() {
        let result = validate_secret_key(None, Some("key2")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidateSecretKeyError::NoKey);
    }

    #[tokio::test]
    async fn empty_secret_key() {
        let result = validate_secret_key(Some(""), Some("key2")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidateSecretKeyError::NoKey);
    }

    #[tokio::test]
    async fn no_expected_key() {
        let result = validate_secret_key(Some("key"), None).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidateSecretKeyError::NoExpectedKey);
    }

    #[tokio::test]
    async fn empty_expected_key() {
        let result = validate_secret_key(Some("key"), Some("")).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), ValidateSecretKeyError::NoExpectedKey);
    }
}
