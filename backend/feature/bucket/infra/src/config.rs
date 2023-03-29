use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub s3_bucket: String,
    pub bucket_base_url: String,
}

impl Config {
    pub fn new() -> Self {
        let s3_bucket = env::var("S3_BUCKET").unwrap_or_else(|_| "just-code-dev".to_string());
        let bucket_base_url = env::var("BUCKET_BASE_URL").unwrap_or_default();

        Self {
            s3_bucket,
            bucket_base_url,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        with_set_values();
        with_no_values();
    }

    fn with_set_values() {
        env::set_var("S3_BUCKET", "just-code-prod");

        let config = Config::new();

        assert_eq!(config.s3_bucket, "just-code-prod");
    }

    fn with_no_values() {
        env::remove_var("S3_BUCKET");

        let config = Config::new();

        assert_eq!(&config.s3_bucket, "just-code-dev");
    }
}
