use content_domain::model::answer_result::AnswerResult;

use crate::{AnswerResultDto, FromModel};

impl FromModel<AnswerResult> for AnswerResultDto {
    fn from_model(model: AnswerResult) -> Self {
        match model {
            AnswerResult::Valid => Self::FirstValid,
            AnswerResult::Invalid => Self::Invalid,
            AnswerResult::AgainValid => Self::Valid,
        }
    }
}
