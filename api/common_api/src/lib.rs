pub mod dto;
mod error;
pub mod lambda;
mod mapper;

common_domain::generate_mapper_traits!();
