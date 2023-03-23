use common_domain::error::{Error, ErrorOutput, ErrorType, Result};

use super::{answer_content::AnswerContent, task::Task, task_content::TaskContent};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Answer {
    pub task_id: String,
    pub content: AnswerContent,
}

impl Answer {
    pub fn is_valid_for(&self, task: &Task) -> Result<bool> {
        match (&task.content, &self.content) {
            (TaskContent::Lesson { content: _ }, AnswerContent::Empty) => Ok(true),
            (
                TaskContent::Playground {
                    content: _,
                    variations: _,
                    dynamic_description: _,
                },
                AnswerContent::Empty,
            ) => Ok(true),
            (
                TaskContent::SingleSelection {
                    content: _,
                    options: _,
                    correct_option,
                    hints: _,
                },
                AnswerContent::SingleAnswer { answer },
            ) => Ok(answer == correct_option),
            (
                TaskContent::MultipleSelection {
                    content: _,
                    options: _,
                    correct_options,
                    hints: _,
                },
                AnswerContent::MultiAnswer { answers },
            ) => Ok(answers == correct_options),
            (
                TaskContent::KeywordsArrangement {
                    content: _,
                    keywords: _,
                    correct_order,
                    hints: _,
                },
                AnswerContent::MultiAnswer { answers },
            ) => Ok(answers == correct_order),
            (
                TaskContent::LinesArrangement {
                    content: _,
                    lines: _,
                    correct_order,
                    hints: _,
                },
                AnswerContent::MultiAnswer { answers },
            ) => Ok(answers == correct_order),
            (
                TaskContent::MissingCode {
                    content: _,
                    correct_code,
                    hints: _,
                },
                AnswerContent::HashMapAnswer { answers },
            ) => Ok(answers == correct_code),
            (_, _) => Err(invalid_answer_type_error()),
        }
    }
}

fn invalid_answer_type_error() -> Error {
    Error {
        debug_message: "Invalid answer type".to_owned(),
        error_type: ErrorType::InvalidInput,
        output: Box::new(ErrorOutput {
            message: "Invalid answer type".to_owned(),
            code: "invalid_answer_type".to_owned(),
            ..Default::default()
        }),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn is_valid_for_lesson_valid() {
        let task = Task {
            content: TaskContent::Lesson {
                content: "content".to_owned(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::Empty,
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_lesson_invalid_type() {
        let task = Task {
            content: TaskContent::Lesson {
                content: "content".to_owned(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer { answers: vec![] },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }

    #[test]
    fn is_valid_for_playground_valid() {
        let task = Task {
            content: TaskContent::Playground {
                content: "content".to_owned(),
                variations: HashMap::new(),
                dynamic_description: HashMap::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::Empty,
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_playground_invalid_type() {
        let task = Task {
            content: TaskContent::Playground {
                content: "content".to_owned(),
                variations: HashMap::new(),
                dynamic_description: HashMap::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer { answers: vec![] },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }

    #[test]
    fn is_valid_for_single_selection_valid() {
        let task = Task {
            content: TaskContent::SingleSelection {
                content: "content".to_owned(),
                options: vec![],
                correct_option: 1,
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::SingleAnswer { answer: 1 },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_single_selection_invalid() {
        let task = Task {
            content: TaskContent::SingleSelection {
                content: "content".to_owned(),
                options: vec![],
                correct_option: 1,
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::SingleAnswer { answer: 2 },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn is_valid_for_single_selection_invalid_type() {
        let task = Task {
            content: TaskContent::SingleSelection {
                content: "content".to_owned(),
                options: vec![],
                correct_option: 1,
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer { answers: vec![] },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }

    #[test]
    fn is_valid_for_multiple_selection_valid() {
        let task = Task {
            content: TaskContent::MultipleSelection {
                content: "content".to_owned(),
                options: vec![],
                correct_options: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer {
                answers: vec![1, 2],
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_multiple_selection_invalid() {
        let task = Task {
            content: TaskContent::MultipleSelection {
                content: "content".to_owned(),
                options: vec![],
                correct_options: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer {
                answers: vec![1, 3],
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn is_valid_for_multiple_selection_invalid_type() {
        let task = Task {
            content: TaskContent::MultipleSelection {
                content: "content".to_owned(),
                options: vec![],
                correct_options: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::SingleAnswer { answer: 1 },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }

    #[test]
    fn is_valid_for_keywords_arrangement_valid() {
        let task = Task {
            content: TaskContent::KeywordsArrangement {
                content: "content".to_owned(),
                keywords: vec![],
                correct_order: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer {
                answers: vec![1, 2],
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_keywords_arrangement_invalid() {
        let task = Task {
            content: TaskContent::KeywordsArrangement {
                content: "content".to_owned(),
                keywords: vec![],
                correct_order: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer {
                answers: vec![1, 3],
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn is_valid_for_keywords_arrangement_invalid_type() {
        let task = Task {
            content: TaskContent::KeywordsArrangement {
                content: "content".to_owned(),
                keywords: vec![],
                correct_order: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::SingleAnswer { answer: 1 },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }

    #[test]
    fn is_valid_for_lines_arrangement_valid() {
        let task = Task {
            content: TaskContent::LinesArrangement {
                content: "content".to_owned(),
                lines: vec![],
                correct_order: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer {
                answers: vec![1, 2],
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_lines_arrangement_invalid() {
        let task = Task {
            content: TaskContent::LinesArrangement {
                content: "content".to_owned(),
                lines: vec![],
                correct_order: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::MultiAnswer {
                answers: vec![1, 3],
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn is_valid_for_lines_arrangement_invalid_type() {
        let task = Task {
            content: TaskContent::LinesArrangement {
                content: "content".to_owned(),
                lines: vec![],
                correct_order: vec![1, 2],
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::SingleAnswer { answer: 1 },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }

    #[test]
    fn is_valid_for_missing_code_valid() {
        let task = Task {
            content: TaskContent::MissingCode {
                content: "content".to_owned(),
                correct_code: HashMap::from([
                    ("a".to_owned(), "b".to_owned()),
                    ("c".to_owned(), "d".to_owned()),
                ]),
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::HashMapAnswer {
                answers: HashMap::from([
                    ("c".to_owned(), "d".to_owned()),
                    ("a".to_owned(), "b".to_owned()),
                ]),
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn is_valid_for_missing_code_invalid() {
        let task = Task {
            content: TaskContent::MissingCode {
                content: "content".to_owned(),
                correct_code: HashMap::from([
                    ("a".to_owned(), "b".to_owned()),
                    ("c".to_owned(), "d".to_owned()),
                ]),
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::HashMapAnswer {
                answers: HashMap::from([
                    ("c".to_owned(), "d".to_owned()),
                    ("a".to_owned(), "b".to_owned()),
                    ("e".to_owned(), "f".to_owned()),
                ]),
            },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn is_valid_for_missing_code_invalid_type() {
        let task = Task {
            content: TaskContent::MissingCode {
                content: "content".to_owned(),
                correct_code: HashMap::from([
                    ("a".to_owned(), "b".to_owned()),
                    ("c".to_owned(), "d".to_owned()),
                ]),
                hints: Vec::new(),
            },
            ..Default::default()
        };
        let answer = Answer {
            content: AnswerContent::SingleAnswer { answer: 1 },
            ..Default::default()
        };

        let result = answer.is_valid_for(&task);

        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), invalid_answer_type_error());
    }
}
