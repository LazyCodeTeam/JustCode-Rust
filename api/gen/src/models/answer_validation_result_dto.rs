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
    #[serde(rename = "result")]
    pub result: crate::models::AnswerResultDto,
}

impl AnswerValidationResultDto {
    pub fn new(result: crate::models::AnswerResultDto) -> AnswerValidationResultDto {
        AnswerValidationResultDto { result }
    }
}
