pub mod dto;
#[cfg(feature = "lambda")]
pub mod lambda;
mod mapper;

common_domain::generate_mapper_traits!();
