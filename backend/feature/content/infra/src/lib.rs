mod config;
mod dto;
pub mod repository;

#[cfg(feature = "dto")]
pub use dto::*;

const TECHNOLOGY_PK: &str = "technology";
const TECHNOLOGY_ID_PREFIX: &str = "technology-";

const SECTION_ID_PREFIX: &str = "section-";

const TASK_GSI_PK: &str = "task";
const TASK_ID_PREFIX: &str = "task-";
const DYNAMIC_TASK_ID_PREFIX: &str = "dynamic_task-";

const POSITIONED_ID_LENGTH: usize = 32;

const TASKS_TRANSACTION_PK: &str = "transaction";
const TASKS_TRANSACTION_SK: &str = "tasks";

const CONTENT_ASSET_PK: &str = "content_asset";

const USER_ANSWER_ID_PREFIX: &str = "user_answer-";

common_domain::generate_mapper_traits!();
