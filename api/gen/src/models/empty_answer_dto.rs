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
pub struct EmptyAnswerDto {
    #[serde(rename = "kind")]
    pub kind: crate::models::AnswerKindDto,
}

impl EmptyAnswerDto {
    pub fn new(kind: crate::models::AnswerKindDto) -> EmptyAnswerDto {
        EmptyAnswerDto { kind }
    }
}
