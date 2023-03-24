use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum AnswerContent {
    #[default]
    Empty,
    SingleAnswer {
        answer: u16,
    },
    MultiAnswer {
        answers: Vec<u16>,
    },
    HashMapAnswer {
        answers: HashMap<String, String>,
    },
}
