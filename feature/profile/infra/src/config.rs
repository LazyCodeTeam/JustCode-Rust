use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub dynamodb_table: String,
}

impl Config {
    pub fn new() -> Self {
        let dynamodb_table =
            env::var("DYNAMODB_TABLE").unwrap_or_else(|_| "just-code-dev".to_string());

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
        env::set_var("DYNAMODB_TABLE", "just-code-prod");

        let config = Config::new();

        assert_eq!(config.dynamodb_table, "just-code-prod");
    }

    fn with_no_values() {
        env::remove_var("DYNAMODB_TABLE");

        let config = Config::new();

        assert_eq!(&config.dynamodb_table, "just-code-dev");
    }
}
