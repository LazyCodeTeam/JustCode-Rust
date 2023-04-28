use content_domain::model::answer_result::AnswerResult;

use crate::{AnswerResultDto, MapFrom};

impl MapFrom<AnswerResult> for AnswerResultDto {
    fn map_from(model: AnswerResult) -> Self {
        match model {
            AnswerResult::Valid => Self::FirstValid,
            AnswerResult::Invalid => Self::Invalid,
            AnswerResult::AgainValid => Self::Valid,
        }
    }
}
