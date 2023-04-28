use content_domain::model::answer_validation_result::AnswerValidationResult;
use gen::models::AnswerValidationResultDto;

use crate::{MapFrom, MapInto};

impl MapFrom<AnswerValidationResult> for AnswerValidationResultDto {
    fn map_from(model: AnswerValidationResult) -> Self {
        AnswerValidationResultDto {
            result: model.result.map_into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::answer_result::AnswerResult;

    use super::*;

    #[test]
    fn map_from() {
        let result = AnswerResult::Valid;
        let model = AnswerValidationResult { result };

        let dto = AnswerValidationResultDto::map_from(model);

        assert_eq!(dto.result, result.map_into());
    }
}
