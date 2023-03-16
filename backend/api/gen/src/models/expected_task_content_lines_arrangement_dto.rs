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
pub struct ExpectedTaskContentLinesArrangementDto {
    #[serde(rename = "kind")]
    pub kind: crate::models::TaskContentKindDto,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "options")]
    pub options: Vec<crate::models::ExpectedOptionDto>,
    #[serde(rename = "correct_order")]
    pub correct_order: Vec<u16>,
    #[serde(rename = "hints")]
    pub hints: Vec<crate::models::HintDto>,
}

impl ExpectedTaskContentLinesArrangementDto {
    pub fn new(
        kind: crate::models::TaskContentKindDto,
        content: String,
        options: Vec<crate::models::ExpectedOptionDto>,
        correct_order: Vec<u16>,
        hints: Vec<crate::models::HintDto>,
    ) -> ExpectedTaskContentLinesArrangementDto {
        ExpectedTaskContentLinesArrangementDto {
            kind,
            content,
            options,
            correct_order,
            hints,
        }
    }
}