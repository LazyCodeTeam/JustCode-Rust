#[cfg(feature = "dynamodb")]
pub mod dynamodb_client;
#[cfg(feature = "dynamodb")]
pub mod dynamodb_identifiable;
pub mod repository;
#[cfg(feature = "s3")]
pub mod s3_client;
#[cfg(feature = "sqs")]
pub mod sqs_client;
pub mod tmp;

pub const DYNAMODB_MAX_BATCH_SIZE: usize = 25;
