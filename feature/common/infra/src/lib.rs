pub mod cognito_client;
pub mod dynamodb_client;
pub mod dynamodb_identifiable;
pub mod repository;
pub mod s3_client;
pub mod sqs_client;
pub mod tmp;

pub const DYNAMODB_MAX_BATCH_SIZE: usize = 25;
