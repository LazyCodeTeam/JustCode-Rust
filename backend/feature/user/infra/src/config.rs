use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub user_pool_id: String,
}

impl Config {
    pub fn new() -> Self {
        let user_pool_id = env::var("USER_POOL_ID").expect("USER_POOL_ID must is not provided");

        Self { user_pool_id }
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
    fn with_set_values() {
        env::set_var("USER_POOL_ID", "user_pool_id");

        let config = Config::new();

        assert_eq!(&config.user_pool_id, "user_pool_id");
    }

    #[test]
    #[should_panic]
    fn with_no_values() {
        env::remove_var("USER_POOL_ID");

        let config = Config::new();

        assert_eq!(&config.user_pool_id, "");
    }
}
