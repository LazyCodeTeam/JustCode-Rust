mod mapper;

pub use gen::models::answer_dto::AnswerDto;
pub use gen::models::answer_result_dto::AnswerResultDto;
pub use gen::models::answer_validation_result_dto::AnswerValidationResultDto;
pub use gen::models::content_asset_dto::ContentAssetDto;
pub use gen::models::expected_keyword_dto::ExpectedKeywordDto;
pub use gen::models::expected_option_dto::ExpectedOptionDto;
pub use gen::models::expected_section_dto::ExpectedSectionDto;
pub use gen::models::expected_task_content_dto::ExpectedTaskContentDto;
pub use gen::models::expected_task_dto::ExpectedTaskDto;
pub use gen::models::expected_technology_dto::ExpectedTechnologyDto;
pub use gen::models::hint_dto::HintDto;
pub use gen::models::keyword_dto::KeywordDto;
pub use gen::models::keyword_modifier_dto::KeywordModifierDto;
pub use gen::models::option_dto::OptionDto;
pub use gen::models::playground_variation_dto::PlaygroundVariationDto;
pub use gen::models::public_task_dto::PublicTaskDto;
pub use gen::models::section_dto::SectionDto;
pub use gen::models::section_preview_dto::SectionPreviewDto;
pub use gen::models::task_content_dto::TaskContentDto;
pub use gen::models::task_preview_dto::TaskPreviewDto;
pub use gen::models::technology_dto::TechnologyDto;

common_domain::generate_mapper_traits!();
