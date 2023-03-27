use content_domain::model::answer_result::AnswerResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnswerResultDto {
    #[default]
    Invalid,
    Valid,
    AgainValid,
}

impl From<AnswerResult> for AnswerResultDto {
    fn from(value: AnswerResult) -> Self {
        match value {
            AnswerResult::Invalid => Self::Invalid,
            AnswerResult::Valid => Self::Valid,
            AnswerResult::AgainValid => Self::AgainValid,
        }
    }
}

impl ToString for AnswerResultDto {
    fn to_string(&self) -> String {
        match self {
            Self::Invalid => "INVALID".to_owned(),
            Self::Valid => "VALID".to_owned(),
            Self::AgainValid => "AGAIN_VALID".to_owned(),
        }
    }
}

impl From<AnswerResultDto> for AnswerResult {
    fn from(value: AnswerResultDto) -> Self {
        match value {
            AnswerResultDto::Invalid => Self::Invalid,
            AnswerResultDto::Valid => Self::Valid,
            AnswerResultDto::AgainValid => Self::AgainValid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use content_domain::model::answer_result::AnswerResult;

    #[test]
    fn to_string() {
        assert_eq!(AnswerResultDto::Invalid.to_string(), "INVALID");
        assert_eq!(AnswerResultDto::Valid.to_string(), "VALID");
        assert_eq!(AnswerResultDto::AgainValid.to_string(), "AGAIN_VALID");
    }

    #[test]
    fn from_historical_answer_result() {
        assert_eq!(
            AnswerResultDto::from(AnswerResult::Invalid),
            AnswerResultDto::Invalid
        );
        assert_eq!(
            AnswerResultDto::from(AnswerResult::Valid),
            AnswerResultDto::Valid
        );
        assert_eq!(
            AnswerResultDto::from(AnswerResult::AgainValid),
            AnswerResultDto::AgainValid
        );
    }

    #[test]
    fn from_answer_result_dto() {
        assert_eq!(
            AnswerResult::from(AnswerResultDto::Invalid),
            AnswerResult::Invalid
        );
        assert_eq!(
            AnswerResult::from(AnswerResultDto::Valid),
            AnswerResult::Valid
        );
        assert_eq!(
            AnswerResult::from(AnswerResultDto::AgainValid),
            AnswerResult::AgainValid
        );
    }
}
