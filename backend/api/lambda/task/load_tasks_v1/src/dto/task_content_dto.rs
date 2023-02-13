use std::collections::HashMap;

use serde::Deserialize;
use validator::Validate;

use super::{
    hint_dto::HintDto, keyword_dto::KeywordDto, option_dto::OptionDto,
    playground_variation_dto::PlaygroundVariationDto,
};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum TaskContentDto {
    Lesson {
        content: String,
    },
    Playground {
        content: String,
        variations: HashMap<String, Vec<PlaygroundVariationDto>>,
        dynamic_description: HashMap<String, String>,
    },
    SingleSelection {
        content: String,
        options: Vec<OptionDto>,
        correct_option: u16,
        hints: Vec<HintDto>,
    },
    MultipleSelection {
        content: String,
        options: Vec<OptionDto>,
        correct_options: Vec<u16>,
        hints: Vec<HintDto>,
    },
    KeywordsArrangement {
        content: String,
        keywords: Vec<KeywordDto>,
        correct_order: Vec<u16>,
        hints: Vec<HintDto>,
    },
    LinesArrangement {
        content: String,
        lines: Vec<OptionDto>,
        correct_order: Vec<u16>,
        hints: Vec<HintDto>,
    },
    MissingCode {
        content: String,
        correct_code: HashMap<String, String>,
        hints: Vec<HintDto>,
    },
}

impl Validate for TaskContentDto {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}
