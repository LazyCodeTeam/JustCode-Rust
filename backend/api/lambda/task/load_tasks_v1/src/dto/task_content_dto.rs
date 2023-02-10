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
        description: String,
        variations: HashMap<String, PlaygroundVariationDto>,
        dynamic_description: HashMap<String, String>,
    },
    SingleSlection {
        content: String,
        options: Vec<OptionDto>,
        correct_option: String,
        hints: Vec<HintDto>,
        difficulty: u8,
    },
    MultipleSelection {
        content: String,
        options: Vec<OptionDto>,
        correct_options: Vec<String>,
        hints: Vec<HintDto>,
        difficulty: u8,
    },
    KeywordsArrangement {
        content: String,
        keywords: Vec<KeywordDto>,
        correct_order: Vec<String>,
        hints: Vec<HintDto>,
        difficulty: u8,
    },
    LinesArrangement {
        content: String,
        lines: Vec<OptionDto>,
        correct_order: Vec<String>,
        hints: Vec<HintDto>,
        difficulty: u8,
    },
    MissingCode {
        content: String,
        correct_code: HashMap<String, String>,
        hints: Vec<HintDto>,
        difficulty: u8,
    },
}

impl Validate for TaskContentDto {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}
