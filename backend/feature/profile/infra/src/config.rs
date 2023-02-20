use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub dynamodb_table: String,
}

impl Config {
    pub fn new() -> Self {
        let dynamodb_table = env::var("PROFILE_DYNAMODB_TABLE")
            .unwrap_or_else(|_| "profile-just-code-dev".to_string());

        Self { dynamodb_table }
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
        env::set_var("PROFILE_DYNAMODB_TABLE", "profile-just-code-prod");

        let config = Config::new();

        assert_eq!(config.dynamodb_table, "profile-just-code-prod");
    }

    fn with_no_values() {
        env::remove_var("PROFILE_DYNAMODB_TABLE");

        let config = Config::new();

        assert_eq!(&config.dynamodb_table, "profile-just-code-dev");
    }
}
