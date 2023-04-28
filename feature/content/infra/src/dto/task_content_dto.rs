use std::collections::HashMap;

use content_domain::model::task_content::TaskContent;
use serde::{Deserialize, Serialize};

use crate::MapInto;

use super::{
    hint_dto::HintDto, keyword_dto::KeywordDto, option_dto::OptionDataDto,
    playground_variation_dto::PlaygroundVariationDto,
};

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
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
        options: Vec<OptionDataDto>,
        correct_option: u16,
        hints: Vec<HintDto>,
    },
    MultipleSelection {
        content: String,
        options: Vec<OptionDataDto>,
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
        lines: Vec<OptionDataDto>,
        correct_order: Vec<u16>,
        hints: Vec<HintDto>,
    },
    MissingCode {
        content: String,
        correct_code: HashMap<String, String>,
        hints: Vec<HintDto>,
    },
}

impl From<TaskContent> for TaskContentDto {
    fn from(task_content: TaskContent) -> Self {
        match task_content {
            TaskContent::Empty => Self::Empty,
            TaskContent::Lesson { content } => Self::Lesson { content },
            TaskContent::Playground {
                content,
                variations,
                dynamic_description,
            } => Self::Playground {
                content,
                variations: variations
                    .into_iter()
                    .map(|(key, value)| (key, value.map_into()))
                    .collect(),
                dynamic_description,
            },
            TaskContent::SingleSelection {
                content,
                options,
                correct_option,
                hints,
            } => Self::SingleSelection {
                content,
                options: options.map_into(),
                correct_option,
                hints: hints.map_into(),
            },
            TaskContent::MultipleSelection {
                content,
                options,
                correct_options,
                hints,
            } => Self::MultipleSelection {
                content,
                options: options.map_into(),
                correct_options,
                hints: hints.map_into(),
            },
            TaskContent::KeywordsArrangement {
                content,
                keywords,
                correct_order,
                hints,
            } => Self::KeywordsArrangement {
                content,
                keywords: keywords.map_into(),
                correct_order,
                hints: hints.map_into(),
            },
            TaskContent::LinesArrangement {
                content,
                lines,
                correct_order,
                hints,
            } => Self::LinesArrangement {
                content,
                lines: lines.map_into(),
                correct_order,
                hints: hints.map_into(),
            },
            TaskContent::MissingCode {
                content,
                correct_code,
                hints,
            } => Self::MissingCode {
                content,
                correct_code,
                hints: hints.map_into(),
            },
        }
    }
}

impl From<TaskContentDto> for TaskContent {
    fn from(task_content_dto: TaskContentDto) -> Self {
        match task_content_dto {
            TaskContentDto::Empty => Self::Empty,
            TaskContentDto::Lesson { content } => Self::Lesson { content },
            TaskContentDto::Playground {
                content,
                variations,
                dynamic_description,
            } => Self::Playground {
                content,
                variations: variations
                    .into_iter()
                    .map(|(key, value)| (key, value.map_into()))
                    .collect(),
                dynamic_description,
            },
            TaskContentDto::SingleSelection {
                content,
                options,
                correct_option,
                hints,
            } => Self::SingleSelection {
                content,
                options: options.map_into(),
                correct_option,
                hints: hints.into_iter().map(MapInto::map_into).collect(),
            },
            TaskContentDto::MultipleSelection {
                content,
                options,
                correct_options,
                hints,
            } => Self::MultipleSelection {
                content,
                options: options.map_into(),
                correct_options,
                hints: hints.map_into(),
            },
            TaskContentDto::KeywordsArrangement {
                content,
                keywords,
                correct_order,
                hints,
            } => Self::KeywordsArrangement {
                content,
                keywords: keywords.map_into(),
                correct_order,
                hints: hints.map_into(),
            },
            TaskContentDto::LinesArrangement {
                content,
                lines,
                correct_order,
                hints,
            } => Self::LinesArrangement {
                content,
                lines: lines.map_into(),
                correct_order,
                hints: hints.map_into(),
            },
            TaskContentDto::MissingCode {
                content,
                correct_code,
                hints,
            } => Self::MissingCode {
                content,
                correct_code,
                hints: hints.map_into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use content_domain::model::{
        hint::Hint, keyword::Keyword, option_data::OptionData,
        playground_variation::PlaygroundVariation,
    };

    use super::*;

    #[test]
    fn from_task_content_empty() {
        let task_content = TaskContent::Empty;
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(task_content_dto, TaskContentDto::Empty);
    }

    #[test]
    fn from_task_content_lesson() {
        let task_content = TaskContent::Lesson {
            content: "content".to_string(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::Lesson {
                content: "content".to_string(),
            }
        );
    }

    #[test]
    fn from_task_content_playground() {
        let variations = HashMap::from([("var".to_string(), vec![PlaygroundVariation::default()])]);
        let dynamic_description = HashMap::from([("var".to_string(), "desc".to_string())]);
        let task_content = TaskContent::Playground {
            content: "content".to_string(),
            variations: variations.clone(),
            dynamic_description: dynamic_description.clone(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::Playground {
                content: "content".to_string(),
                variations: variations
                    .into_iter()
                    .map(|(key, value)| (key, value.map_into()))
                    .collect(),
                dynamic_description,
            }
        );
    }

    #[test]
    fn from_task_content_single_selection() {
        let options = vec![OptionData::default()];
        let hints = vec![Hint::default()];
        let task_content = TaskContent::SingleSelection {
            content: "content".to_string(),
            options: options.clone(),
            correct_option: 0,
            hints: hints.clone(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::SingleSelection {
                content: "content".to_string(),
                options: options.map_into(),
                correct_option: 0,
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_multiple_selection() {
        let options = vec![OptionData::default()];
        let hints = vec![Hint::default()];
        let task_content = TaskContent::MultipleSelection {
            content: "content".to_string(),
            options: options.clone(),
            correct_options: vec![0],
            hints: hints.clone(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::MultipleSelection {
                content: "content".to_string(),
                options: options.map_into(),
                correct_options: vec![0],
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_keywords_arrangement() {
        let keywords = vec![Keyword::default()];
        let hints = vec![Hint::default()];
        let task_content = TaskContent::KeywordsArrangement {
            content: "content".to_string(),
            keywords: keywords.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::KeywordsArrangement {
                content: "content".to_string(),
                keywords: keywords.map_into(),
                correct_order: vec![0],
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_lines_arrangement() {
        let lines = vec![OptionData::default()];
        let hints = vec![Hint::default()];
        let task_content = TaskContent::LinesArrangement {
            content: "content".to_string(),
            lines: lines.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::LinesArrangement {
                content: "content".to_string(),
                lines: lines.map_into(),
                correct_order: vec![0],
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_missing_code() {
        let hints = vec![Hint::default()];
        let correct_code = HashMap::from([("var".to_string(), "code".to_string())]);
        let task_content = TaskContent::MissingCode {
            content: "content".to_string(),
            correct_code: correct_code.clone(),
            hints: hints.clone(),
        };
        let task_content_dto = TaskContentDto::from(task_content);
        assert_eq!(
            task_content_dto,
            TaskContentDto::MissingCode {
                content: "content".to_string(),
                correct_code,
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_lesson_dto() {
        let task_content_dto = TaskContentDto::Lesson {
            content: "content".to_string(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::Lesson {
                content: "content".to_string(),
            }
        );
    }

    #[test]
    fn from_task_content_playground_dto() {
        let variations =
            HashMap::from([("var".to_string(), vec![PlaygroundVariationDto::default()])]);
        let dynamic_description = HashMap::from([("var".to_string(), "desc".to_string())]);
        let task_content_dto = TaskContentDto::Playground {
            content: "content".to_string(),
            variations: variations.clone(),
            dynamic_description: dynamic_description.clone(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::Playground {
                content: "content".to_string(),
                variations: variations
                    .into_iter()
                    .map(|(key, value)| (key, value.map_into()))
                    .collect(),
                dynamic_description,
            }
        );
    }

    #[test]
    fn from_task_content_single_selection_dto() {
        let options = vec![OptionDataDto::default()];
        let hints = vec![HintDto::default()];
        let task_content_dto = TaskContentDto::SingleSelection {
            content: "content".to_string(),
            options: options.clone(),
            correct_option: 0,
            hints: hints.clone(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::SingleSelection {
                content: "content".to_string(),
                options: options.map_into(),
                correct_option: 0,
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_multiple_selection_dto() {
        let options = vec![OptionDataDto::default()];
        let hints = vec![HintDto::default()];
        let task_content_dto = TaskContentDto::MultipleSelection {
            content: "content".to_string(),
            options: options.clone(),
            correct_options: vec![0],
            hints: hints.clone(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::MultipleSelection {
                content: "content".to_string(),
                options: options.map_into(),
                correct_options: vec![0],
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_keywords_arrangement_dto() {
        let keywords = vec![KeywordDto::default()];
        let hints = vec![HintDto::default()];
        let task_content_dto = TaskContentDto::KeywordsArrangement {
            content: "content".to_string(),
            keywords: keywords.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::KeywordsArrangement {
                content: "content".to_string(),
                keywords: keywords.map_into(),
                correct_order: vec![0],
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_lines_arrangement_dto() {
        let lines = vec![OptionDataDto::default()];
        let hints = vec![HintDto::default()];
        let task_content_dto = TaskContentDto::LinesArrangement {
            content: "content".to_string(),
            lines: lines.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::LinesArrangement {
                content: "content".to_string(),
                lines: lines.map_into(),
                correct_order: vec![0],
                hints: hints.map_into(),
            }
        );
    }

    #[test]
    fn from_task_content_missing_code_dto() {
        let hints = vec![HintDto::default()];
        let correct_code = HashMap::from([("var".to_string(), "code".to_string())]);
        let task_content_dto = TaskContentDto::MissingCode {
            content: "content".to_string(),
            correct_code: correct_code.clone(),
            hints: hints.clone(),
        };

        let task_content = TaskContent::from(task_content_dto);

        assert_eq!(
            task_content,
            TaskContent::MissingCode {
                content: "content".to_string(),
                correct_code,
                hints: hints.map_into(),
            }
        );
    }
}
