use serde::{Deserialize, Serialize};
use task_domain::model::tasks_transaction_state::TasksTransactionState;

use crate::{TASKS_TRANSACTION_PK, TASKS_TRANSACTION_SK};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TasksTransactionDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub items_count: u64,
    pub items_passed_to_queue_count: u64,
    pub processed_items_count: u64,
}

impl TasksTransactionDto {
    pub fn new(items_count: u64) -> Self {
        Self {
            pk: TASKS_TRANSACTION_PK.to_owned(),
            sk: TASKS_TRANSACTION_SK.to_owned(),
            items_count,
            items_passed_to_queue_count: 0,
            processed_items_count: 0,
        }
    }
}

impl From<TasksTransactionDto> for TasksTransactionState {
    fn from(value: TasksTransactionDto) -> Self {
        match (
            value.items_passed_to_queue_count,
            value.processed_items_count,
        ) {
            (0, 0) => TasksTransactionState::PopulatingQueue,
            (items_passed_to_queue_count, 0)
                if items_passed_to_queue_count <= value.items_count =>
            {
                TasksTransactionState::QueuePopulated
            }
            (items_passed_to_queue_count, processed_items_count)
                if items_passed_to_queue_count == value.items_count
                    && processed_items_count <= value.items_count =>
            {
                TasksTransactionState::ProcessingQueue
            }
            _ => TasksTransactionState::Invalid,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_new_object() {
        let expected = TasksTransactionDto {
            pk: TASKS_TRANSACTION_PK.to_owned(),
            sk: TASKS_TRANSACTION_SK.to_owned(),
            items_count: 10,
            items_passed_to_queue_count: 0,
            processed_items_count: 0,
        };
        let actual = TasksTransactionDto::new(10);

        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_populating_queue_state() {
        let dto = TasksTransactionDto {
            pk: TASKS_TRANSACTION_PK.to_owned(),
            sk: TASKS_TRANSACTION_SK.to_owned(),
            items_count: 10,
            items_passed_to_queue_count: 0,
            processed_items_count: 0,
        };

        let actual = TasksTransactionState::from(dto);

        assert_eq!(TasksTransactionState::PopulatingQueue, actual);
    }

    #[test]
    fn should_return_queue_populated_state() {
        let dto = TasksTransactionDto {
            pk: TASKS_TRANSACTION_PK.to_owned(),
            sk: TASKS_TRANSACTION_SK.to_owned(),
            items_count: 10,
            items_passed_to_queue_count: 5,
            processed_items_count: 0,
        };

        let actual = TasksTransactionState::from(dto);

        assert_eq!(TasksTransactionState::QueuePopulated, actual);
    }

    #[test]
    fn should_return_processing_queue_state() {
        let dto = TasksTransactionDto {
            pk: TASKS_TRANSACTION_PK.to_owned(),
            sk: TASKS_TRANSACTION_SK.to_owned(),
            items_count: 10,
            items_passed_to_queue_count: 10,
            processed_items_count: 5,
        };

        let actual = TasksTransactionState::from(dto);

        assert_eq!(TasksTransactionState::ProcessingQueue, actual);
    }

    #[test]
    fn should_return_invalid_state() {
        let dto = TasksTransactionDto {
            pk: TASKS_TRANSACTION_PK.to_owned(),
            sk: TASKS_TRANSACTION_SK.to_owned(),
            items_count: 10,
            items_passed_to_queue_count: 10,
            processed_items_count: 11,
        };

        let actual = TasksTransactionState::from(dto);

        assert_eq!(TasksTransactionState::Invalid, actual);
    }
}
