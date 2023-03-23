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
pub struct AnswerKeywordsArrangementDto {
    #[serde(rename = "kind")]
    pub kind: crate::models::TaskContentKindDto,
    #[serde(rename = "answer")]
    pub answer: Vec<u16>,
}

impl AnswerKeywordsArrangementDto {
    pub fn new(
        kind: crate::models::TaskContentKindDto,
        answer: Vec<u16>,
    ) -> AnswerKeywordsArrangementDto {
        AnswerKeywordsArrangementDto { kind, answer }
    }
}
