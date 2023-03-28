mod config;
mod dto;
pub mod repository;

const PROFILE_ID_PREFIX: &str = "profile-";
const PROFILE_SORT_KEY: &str = "profile";

common_domain::generate_mapper_traits!();
