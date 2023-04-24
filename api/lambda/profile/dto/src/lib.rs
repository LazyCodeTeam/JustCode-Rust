mod mapper;

pub use gen::models::*;

common_domain::generate_mapper_traits!();
