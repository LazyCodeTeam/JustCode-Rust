use std::env;

lazy_static::lazy_static! {
    pub static ref CONFIG: Config = Config::default();
}

pub struct Config {
    pub task_migration_sqs_queue: String,
    pub dynamodb_table: String,
    pub content_dynamodb_table: String,
}

impl Config {
    pub fn new() -> Self {
        let task_migration_sqs_queue =
            env::var("TASK_MIGRATION_SQS_QUEUE").unwrap_or_else(|_| "".to_string());
        let dynamodb_table = env::var("DYNAMODB_TABLE").unwrap_or_else(|_| "".to_string());
        let content_dynamodb_table =
            env::var("CONTENT_DYNAMODB_TABLE").unwrap_or_else(|_| "".to_string());

        Self {
            task_migration_sqs_queue,
            dynamodb_table,
            content_dynamodb_table,
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
        env::set_var("DYNAMODB_TABLE", "just-code-prod");

        let config = Config::new();

        assert_eq!(
            config.task_migration_sqs_queue,
            "task-migration-just-code-prod"
        );
        assert_eq!(config.dynamodb_table, "just-code-prod");
    }

    fn with_no_values() {
        env::remove_var("TASK_MIGRATION_SQS_QUEUE");
        env::remove_var("DYNAMODB_TABLE");

        let config = Config::new();

        assert_eq!(&config.task_migration_sqs_queue, "");
        assert_eq!(&config.dynamodb_table, "");
    }
}
