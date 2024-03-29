use content_domain::model::task_content::TaskContent;

use crate::{ExpectedTaskContentDto, MapFrom, MapInto};

impl MapFrom<Option<ExpectedTaskContentDto>> for TaskContent {
    fn map_from(dto: Option<ExpectedTaskContentDto>) -> Self {
        match dto {
            Some(dto) => dto.map_into(),
            None => TaskContent::Empty,
        }
    }
}

impl MapFrom<ExpectedTaskContentDto> for TaskContent {
    fn map_from(dto: ExpectedTaskContentDto) -> Self {
        match dto {
            ExpectedTaskContentDto::TaskContentLessonDto { content } => {
                TaskContent::Lesson { content }
            }

            ExpectedTaskContentDto::TaskContentPlaygroundDto {
                content,
                variations,
                dynamic_content,
            } => TaskContent::Playground {
                content,
                variations: variations
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().map(MapInto::map_into).collect()))
                    .collect(),
                dynamic_description: dynamic_content,
            },

            ExpectedTaskContentDto::ExpectedTaskContentSingleSelectionDto {
                content,
                options,
                correct_option,
                hints,
            } => TaskContent::SingleSelection {
                content,
                options: options.map_into(),
                correct_option,
                hints: hints.into_iter().map(MapInto::map_into).collect(),
            },

            ExpectedTaskContentDto::ExpectedTaskContentMultipleSelectionDto {
                content,
                options,
                correct_options,
                hints,
            } => TaskContent::MultipleSelection {
                content,
                options: options.map_into(),
                correct_options,
                hints: hints.into_iter().map(MapInto::map_into).collect(),
            },

            ExpectedTaskContentDto::ExpectedTaskContentKeywordsArrangementDto {
                content,
                keywords,
                correct_order,
                hints,
            } => TaskContent::KeywordsArrangement {
                content,
                keywords: keywords.map_into(),
                correct_order,
                hints: hints.into_iter().map(MapInto::map_into).collect(),
            },

            ExpectedTaskContentDto::ExpectedTaskContentLinesArrangementDto {
                content,
                options,
                correct_order,
                hints,
            } => TaskContent::LinesArrangement {
                content,
                lines: options.map_into(),
                correct_order,
                hints: hints.into_iter().map(MapInto::map_into).collect(),
            },

            ExpectedTaskContentDto::TaskContentMissingCodeDto {
                content,
                correct_code,
                hints,
            } => TaskContent::MissingCode {
                content,
                correct_code,
                hints: hints.into_iter().map(MapInto::map_into).collect(),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ExpectedKeywordDto, ExpectedOptionDto, HintDto, PlaygroundVariationDto};
    use std::collections::HashMap;

    #[test]
    fn option_task_content_none() {
        let dto = None;
        let content: TaskContent = dto.map_into();
        assert_eq!(content, TaskContent::Empty);
    }

    #[test]
    fn test_task_content_dto_lesson() {
        let dto = ExpectedTaskContentDto::TaskContentLessonDto {
            content: "content".to_string(),
        };
        let content: TaskContent = dto.map_into();
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
        let dto = ExpectedTaskContentDto::TaskContentPlaygroundDto {
            content: "content".to_string(),
            variations: variations.clone(),
            dynamic_content: HashMap::from([("key".to_string(), "description".to_string())]),
        };
        let content: TaskContent = dto.map_into();
        assert_eq!(
            content,
            TaskContent::Playground {
                content: "content".to_string(),
                variations: variations
                    .into_iter()
                    .map(|(k, v)| (k, v.into_iter().map(MapInto::map_into).collect()))
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
        let options = vec![ExpectedOptionDto {
            content: "content".to_string(),
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let dto = ExpectedTaskContentDto::ExpectedTaskContentSingleSelectionDto {
            content: "content".to_string(),
            options: options.clone(),
            correct_option: 0,
            hints: hints.clone(),
        };
        let content: TaskContent = dto.map_into();
        assert_eq!(
            content,
            TaskContent::SingleSelection {
                content: "content".to_string(),
                options: options.map_into(),
                correct_option: 0,
                hints: hints.into_iter().map(MapInto::map_into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_multiple_selection() {
        let options = vec![ExpectedOptionDto {
            content: "content".to_string(),
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let dto = ExpectedTaskContentDto::ExpectedTaskContentMultipleSelectionDto {
            content: "content".to_string(),
            options: options.clone(),
            correct_options: vec![0],
            hints: hints.clone(),
        };
        let content: TaskContent = dto.map_into();
        assert_eq!(
            content,
            TaskContent::MultipleSelection {
                content: "content".to_string(),
                options: options.map_into(),
                correct_options: vec![0],
                hints: hints.into_iter().map(MapInto::map_into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_keywords_arrangement() {
        let keywords = vec![ExpectedKeywordDto {
            content: "content".to_string(),
            modifiers: vec![],
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];

        let dto = ExpectedTaskContentDto::ExpectedTaskContentKeywordsArrangementDto {
            content: "content".to_string(),
            keywords: keywords.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };
        let content: TaskContent = dto.map_into();
        assert_eq!(
            content,
            TaskContent::KeywordsArrangement {
                content: "content".to_string(),
                keywords: keywords.map_into(),
                correct_order: vec![0],
                hints: hints.into_iter().map(MapInto::map_into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_lines_arrangement() {
        let lines = vec![ExpectedOptionDto {
            content: "content".to_string(),
        }];
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let dto = ExpectedTaskContentDto::ExpectedTaskContentLinesArrangementDto {
            content: "content".to_string(),
            options: lines.clone(),
            correct_order: vec![0],
            hints: hints.clone(),
        };

        let content: TaskContent = dto.map_into();

        assert_eq!(
            content,
            TaskContent::LinesArrangement {
                content: "content".to_string(),
                lines: lines.map_into(),
                correct_order: vec![0],
                hints: hints.into_iter().map(MapInto::map_into).collect()
            }
        );
    }

    #[test]
    fn test_task_content_dto_missing_code() {
        let hints = vec![HintDto {
            content: "content".to_string(),
        }];
        let correct_code = HashMap::from([("key".to_string(), "code".to_string())]);
        let dto = ExpectedTaskContentDto::TaskContentMissingCodeDto {
            content: "content".to_string(),
            correct_code: correct_code.clone(),
            hints: hints.clone(),
        };
        let content: TaskContent = dto.map_into();
        assert_eq!(
            content,
            TaskContent::MissingCode {
                content: "content".to_string(),
                correct_code,
                hints: hints.into_iter().map(MapInto::map_into).collect()
            }
        );
    }
}
