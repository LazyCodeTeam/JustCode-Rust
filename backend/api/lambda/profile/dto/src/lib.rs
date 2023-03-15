mod mapper;

pub use gen::models::platform_dto::PlatformDto;
pub use gen::models::profile_dto::ProfileDto;
pub use gen::models::push_data_dto::PushDataDto;
pub use gen::models::update_profile_dto::UpdateProfileDto;

common_domain::generate_mapper_traits!();
