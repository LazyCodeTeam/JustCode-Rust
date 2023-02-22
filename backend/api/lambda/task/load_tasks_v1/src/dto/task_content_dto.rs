use std::collections::HashMap;

use serde::Deserialize;
use task_domain::model::task_content::TaskContent;
use validator::Validate;

use super::{
    hint_dto::HintDto,
    keyword_dto::{KeywordDto, Keywords},
    option_dto::{OptionDto, OptionsData},
    playground_variation_dto::PlaygroundVariationDto,
};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Default)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskContentDto {
    #[default]
    Empty, // For testing only
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

impl From<TaskContentDto> for TaskContent {
    fn from(value: TaskContentDto) -> Self {
        match value {
            TaskContentDto::Empty => TaskContent::Empty,

            TaskContentDto::Lesson { content } => TaskContent::Lesson { content },

            TaskContentDto::Playground {
                content,
                variations,
                dynamic_description,
            } => TaskContent::Playground {
                content,
                variations: variations
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().map(Into::into).collect()))
                    .collect(),
                dynamic_description,
            },

            TaskContentDto::SingleSelection {
                content,
                options,
                correct_option,
                hints,
            } => TaskContent::SingleSelection {
                content,
                options: OptionsData::from(options).into(),
                correct_option,
                hints: hints.into_iter().map(Into::into).collect(),
            },

            TaskContentDto::MultipleSelection {
                content,
                options,
                correct_options,
                hints,
            } => TaskContent::MultipleSelection {
                content,
                options: OptionsData::from(options).into(),
                correct_options,
                hints: hints.into_iter().map(Into::into).collect(),
            },

            TaskContentDto::KeywordsArrangement {
                content,
                keywords,
                correct_order,
                hints,
            } => TaskContent::KeywordsArrangement {
                content,
                keywords: Keywords::from(keywords).into(),
                correct_order,
                hints: hints.into_iter().map(Into::into).collect(),
            },

            TaskContentDto::LinesArrangement {
                content,
                lines,
                correct_order,
                hints,
            } => TaskContent::LinesArrangement {
                content,
                lines: OptionsData::from(lines).into(),
                correct_order,
                hints: hints.into_iter().map(Into::into).collect(),
            },

            TaskContentDto::MissingCode {
                content,
                correct_code,
                hints,
            } => TaskContent::MissingCode {
                content,
                correct_code,
                hints: hints.into_iter().map(Into::into).collect(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_task_content_dto() {
        let dto = TaskContentDto::Empty;
        let content: TaskContent = dto.into();
        assert_eq!(content, TaskContent::Empty);
    }

    #[test]
    fn test_task_content_dto_lesson() {
        let dto = TaskContentDto::Lesson {
            content: "content".to_string(),
        };
        let content: TaskContent = dto.into();
        assert_eq!(
            content,
            TaskContent::Lesson {
                content: "content".to_string()
            }
        );
    }

    #[test]
    fn test_task_content_dto_playground() {
        let variations = vec![PlaygroundVariationDto {
            content: "content".to_string(),
            description: "description".to_string(),
        }];
        let variations = HashMap::from([("key".to_string(), variations)]);
        let dto = TaskContentDto::Playground {
            content: "content".to_string(),
            variations: variations.clone(),
            dynamic_description: HashMap::from([("key".to_string(), "description".to_string())]),
        };
        let content: TaskContent = dto.into();
        assert_eq!(
            content,
            TaskContent::Playground {
                content: "content".to_string(),
                variations: variations
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().map(Into::into).collect()))
                    .collect(),
                dynamic_description: HashMap::from([(
                    "key".to_string(),
                    "description".to_string()
                )]),
            }
        );
    }

    #[test]
    fn test_task_content_dto_single_selection() {
        let options = vec![OptionDto {
            content: "content".to_string(),
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let dto = TaskContentDto::SingleSelection {
            content: "content".to_string(),
            options: options.clone(),
            correct_option: 0,
            hints: hints.clone(),
        };
        let content: TaskContent = dto.into();
        assert_eq!(
            content,
            TaskContent::SingleSelection {
                content: "content".to_string(),
                options: OptionsData::from(options).into(),
                correct_option: 0,
                hints: hints.into_iter().map(Into::into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_multiple_selection() {
        let options = vec![OptionDto {
            content: "content".to_string(),
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let dto = TaskContentDto::MultipleSelection {
            content: "content".to_string(),
            options: options.clone(),
            correct_options: vec![0],
            hints: hints.clone(),
        };
        let content: TaskContent = dto.into();
        assert_eq!(
            content,
            TaskContent::MultipleSelection {
                content: "content".to_string(),
                options: OptionsData::from(options).into(),
                correct_options: vec![0],
                hints: hints.into_iter().map(Into::into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_keywords_arrangement() {
        let keywords = vec![KeywordDto {
            content: "content".to_string(),
            modifiers: vec![],
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];

        let dto = TaskContentDto::KeywordsArrangement {
            content: "content".to_string(),
            keywords: keywords.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };
        let content: TaskContent = dto.into();
        assert_eq!(
            content,
            TaskContent::KeywordsArrangement {
                content: "content".to_string(),
                keywords: Keywords::from(keywords).into(),
                correct_order: vec![0],
                hints: hints.into_iter().map(Into::into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_lines_arrangement() {
        let lines = vec![OptionDto {
            content: "content".to_string(),
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let dto = TaskContentDto::LinesArrangement {
            content: "content".to_string(),
            lines: lines.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };

        let content: TaskContent = dto.into();

        assert_eq!(
            content,
            TaskContent::LinesArrangement {
                content: "content".to_string(),
                lines: OptionsData::from(lines).into(),
                correct_order: vec![0],
                hints: hints.into_iter().map(Into::into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_missing_code() {
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let correct_code = HashMap::from([("key".to_string(), "code".to_string())]);
        let dto = TaskContentDto::MissingCode {
            content: "content".to_string(),
            correct_code: correct_code.clone(),
            hints: hints.clone(),
        };
        let content: TaskContent = dto.into();
        assert_eq!(
            content,
            TaskContent::MissingCode {
                content: "content".to_string(),
                correct_code,
                hints: hints.into_iter().map(Into::into).collect()
            }
        );
    }
}
