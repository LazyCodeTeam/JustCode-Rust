use content_domain::model::answer_validation_result::AnswerValidationResult;
use gen::models::AnswerValidationResultDto;

use crate::{FromModel, IntoDto};

impl FromModel<AnswerValidationResult> for AnswerValidationResultDto {
    fn from_model(model: AnswerValidationResult) -> Self {
        AnswerValidationResultDto {
            result: model.result.into_dto(),
        }
    }
}

#[cfg(test)]
mod tests {
    use content_domain::model::answer_result::AnswerResult;

    use super::*;

    #[test]
    fn from_model() {
        let result = AnswerResult::Valid;
        let model = AnswerValidationResult { result };

        let dto = AnswerValidationResultDto::from_model(model);

        assert_eq!(dto.result, result.into_dto());
    }
}
