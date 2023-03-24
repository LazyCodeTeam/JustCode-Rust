/*
 * just-code-dev
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2023-03-10 06:01:08UTC
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct AnswerValidationResultDto {
    #[serde(rename = "is_valid")]
    pub is_valid: bool,
    #[serde(rename = "had_valid_answer_before")]
    pub had_valid_answer_before: bool,
}

impl AnswerValidationResultDto {
    pub fn new(is_valid: bool, had_valid_answer_before: bool) -> AnswerValidationResultDto {
        AnswerValidationResultDto {
            is_valid,
            had_valid_answer_before,
        }
    }
}
