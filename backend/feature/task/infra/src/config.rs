use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub task_migration_sqs_queuq: String,
}

impl Config {
    pub fn new() -> Self {
        let task_migration_sqs_queuq =
            env::var("TASK_MIGRATION_SQS_QUEUE").unwrap_or_else(|_| "".to_string());

        Self {
            task_migration_sqs_queuq,
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
        env::set_var("TASK_MIGRATION_SQS_QUEUE", "task-migration-just-code-prod");

        let config = Config::new();

        assert_eq!(
            config.task_migration_sqs_queuq,
            "task-migration-just-code-prod"
        );
    }

    fn with_no_values() {
        env::remove_var("TASK_MIGRATION_SQS_QUEUE");

        let config = Config::new();

        assert_eq!(&config.task_migration_sqs_queuq, "");
    }
}
