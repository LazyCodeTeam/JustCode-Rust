use content_domain::model::answer_validation_result::AnswerValidationResult;
use gen::models::AnswerValidationResultDto;

use crate::FromModel;

impl FromModel<AnswerValidationResult> for AnswerValidationResultDto {
    fn from_model(model: AnswerValidationResult) -> Self {
        AnswerValidationResultDto {
            is_valid: model.is_valid,
            had_valid_answer_before: model.had_valid_answer_before,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_model() {
        let model = AnswerValidationResult {
            is_valid: true,
            had_valid_answer_before: false,
        };

        let dto = AnswerValidationResultDto::from_model(model);

        assert!(dto.is_valid);
        assert!(!dto.had_valid_answer_before);
    }
}
