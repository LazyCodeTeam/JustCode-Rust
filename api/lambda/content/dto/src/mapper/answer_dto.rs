use crate::AnswerDto;
use content_domain::model::{answer::Answer, answer_content::AnswerContent};

use crate::{MapFrom, MapInto};

impl MapFrom<(String, AnswerDto)> for Answer {
    fn map_from((task_id, dto): (String, AnswerDto)) -> Self {
        Answer {
            task_id,
            content: dto.map_into(),
        }
    }
}

impl MapFrom<AnswerDto> for AnswerContent {
    fn map_from(dto: AnswerDto) -> Self {
        match dto {
            AnswerDto::EmptyAnswerDto {} => AnswerContent::Empty,
            AnswerDto::SingleAnswerDto { answer } => AnswerContent::SingleAnswer { answer },
            AnswerDto::MultiAnswersDto { answer } => AnswerContent::MultiAnswer { answers: answer },
            AnswerDto::HashMapAnswersDto { answer } => {
                AnswerContent::HashMapAnswer { answers: answer }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn from_answer_dto_with_id() {
        let dto = AnswerDto::SingleAnswerDto { answer: 1 };

        let answer = Answer::map_from(("task_id".to_owned(), dto));

        assert_eq!(
            answer,
            Answer {
                task_id: "task_id".to_string(),
                content: AnswerContent::SingleAnswer { answer: 1 }
            }
        );
    }

    #[test]
    fn from_empyt_answer_dto() {
        let dto = AnswerDto::EmptyAnswerDto {};

        let answer_content = AnswerContent::map_from(dto);

        assert_eq!(answer_content, AnswerContent::Empty);
    }

    #[test]
    fn from_single_answer_dto() {
        let dto = AnswerDto::SingleAnswerDto { answer: 1 };

        let answer_content = AnswerContent::map_from(dto);

        assert_eq!(answer_content, AnswerContent::SingleAnswer { answer: 1 });
    }

    #[test]
    fn from_multi_answer_dto() {
        let dto = AnswerDto::MultiAnswersDto {
            answer: vec![1, 2, 3],
        };

        let answer_content = AnswerContent::map_from(dto);

        assert_eq!(
            answer_content,
            AnswerContent::MultiAnswer {
                answers: vec![1, 2, 3]
            }
        );
    }

    #[test]
    fn from_hash_map_answer_dto() {
        let map = HashMap::from([("key".to_string(), "value".to_string())]);
        let dto = AnswerDto::HashMapAnswersDto { answer: map };

        let answer_content = AnswerContent::map_from(dto);

        assert_eq!(
            answer_content,
            AnswerContent::HashMapAnswer {
                answers: HashMap::from([("key".to_string(), "value".to_string())])
            }
        );
    }
}
