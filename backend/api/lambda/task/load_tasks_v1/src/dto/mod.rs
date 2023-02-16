pub mod hint_dto;
pub mod keyword_dto;
pub mod option_dto;
pub mod playground_variation_dto;
pub mod section_dto;
pub mod task_content_dto;
pub mod task_dto;
pub mod technology_dto;

lazy_static::lazy_static! {
    static ref UUID_PATTERN: regex::Regex = regex::Regex::new(
        r"^[0-9a-fA-F]{8}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{4}\b-[0-9a-fA-F]{12}$"
    )
    .unwrap();
}
