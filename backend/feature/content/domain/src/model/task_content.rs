use std::collections::HashMap;

use super::{
    hint::Hint, keyword::Keyword, option_data::OptionData,
    playground_variation::PlaygroundVariation,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum TaskContent {
    #[default]
    Empty, // For testing only
    Lesson {
        content: String,
    },
    Playground {
        content: String,
        variations: HashMap<String, Vec<PlaygroundVariation>>,
        dynamic_description: HashMap<String, String>,
    },
    SingleSelection {
        content: String,
        options: Vec<OptionData>,
        correct_option: u16,
        hints: Vec<Hint>,
    },
    MultipleSelection {
        content: String,
        options: Vec<OptionData>,
        correct_options: Vec<u16>,
        hints: Vec<Hint>,
    },
    KeywordsArrangement {
        content: String,
        keywords: Vec<Keyword>,
        correct_order: Vec<u16>,
        hints: Vec<Hint>,
    },
    LinesArrangement {
        content: String,
        lines: Vec<OptionData>,
        correct_order: Vec<u16>,
        hints: Vec<Hint>,
    },
    MissingCode {
        content: String,
        correct_code: HashMap<String, String>,
        hints: Vec<Hint>,
    },
}
