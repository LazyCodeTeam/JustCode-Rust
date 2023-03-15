use crate::{FromModel, IntoDto, TaskContentDto};
use content_domain::model::task_content::TaskContent;

impl FromModel<TaskContent> for Option<TaskContentDto> {
    fn from_model(model: TaskContent) -> Self {
        match model {
            TaskContent::Empty => None,
            TaskContent::Lesson { content } => {
                Some(TaskContentDto::TaskContentLessonDto { content })
            }
            TaskContent::Playground {
                content,
                variations,
                dynamic_description,
            } => Some(TaskContentDto::TaskContentPlaygroundDto {
                content,
                variations: variations
                    .into_iter()
                    .map(|(key, value)| (key, value.into_iter().map(IntoDto::into_dto).collect()))
                    .collect(),
                dynamic_content: dynamic_description,
            }),
            TaskContent::SingleSelection {
                content,
                options,
                correct_option,
                hints,
            } => Some(TaskContentDto::TaskContentSingleSelectionDto {
                content,
                options: options.into_iter().map(IntoDto::into_dto).collect(),
                correct_option,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
            }),
            TaskContent::MultipleSelection {
                content,
                options,
                correct_options,
                hints,
            } => Some(TaskContentDto::TaskContentMultipleSelectionDto {
                content,
                options: options.into_iter().map(IntoDto::into_dto).collect(),
                correct_options,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
            }),
            TaskContent::KeywordsArrangement {
                content,
                keywords,
                correct_order,
                hints,
            } => Some(TaskContentDto::TaskContentKeywordsArrangementDto {
                content,
                keywords: keywords.into_iter().map(IntoDto::into_dto).collect(),
                correct_order,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
            }),
            TaskContent::LinesArrangement {
                content,
                lines,
                correct_order,
                hints,
            } => Some(TaskContentDto::TaskContentLinesArrangementDto {
                content,
                options: lines.into_iter().map(IntoDto::into_dto).collect(),
                correct_order,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
            }),
            TaskContent::MissingCode {
                content,
                correct_code,
                hints,
            } => Some(TaskContentDto::TaskContentMissingCodeDto {
                content,
                correct_code,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use content_domain::model::{
        hint::Hint, keyword::Keyword, option_data::OptionData,
        playground_variation::PlaygroundVariation,
    };

    use super::*;

    #[test]
    fn from_task_content_empty() {
        let task_content = TaskContent::Empty;
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert!(task_content_dto.is_none());
    }

    #[test]
    fn from_task_content_lesson() {
        let task_content = TaskContent::Lesson {
            content: "content".to_string(),
        };
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentLessonDto {
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
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentPlaygroundDto {
                content: "content".to_string(),
                variations: variations
                    .into_iter()
                    .map(|(key, value)| (key, value.into_iter().map(IntoDto::into_dto).collect()))
                    .collect(),
                dynamic_content: dynamic_description,
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
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentSingleSelectionDto {
                content: "content".to_string(),
                options: options.into_iter().map(IntoDto::into_dto).collect(),
                correct_option: 0,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
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
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentMultipleSelectionDto {
                content: "content".to_string(),
                options: options.into_iter().map(IntoDto::into_dto).collect(),
                correct_options: vec![0],
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
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
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentKeywordsArrangementDto {
                content: "content".to_string(),
                keywords: keywords.into_iter().map(IntoDto::into_dto).collect(),
                correct_order: vec![0],
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
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
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentLinesArrangementDto {
                content: "content".to_string(),
                options: lines.into_iter().map(IntoDto::into_dto).collect(),
                correct_order: vec![0],
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
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
        let task_content_dto = Option::<TaskContentDto>::from_model(task_content);
        assert_eq!(
            task_content_dto.unwrap(),
            TaskContentDto::TaskContentMissingCodeDto {
                content: "content".to_string(),
                correct_code,
                hints: hints.into_iter().map(IntoDto::into_dto).collect(),
            }
        );
    }
}
