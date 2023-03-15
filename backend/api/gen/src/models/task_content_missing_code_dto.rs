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
pub struct TaskContentMissingCodeDto {
    #[serde(rename = "kind")]
    pub kind: crate::models::TaskContentKindDto,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "correct_code")]
    pub correct_code: ::std::collections::HashMap<String, String>,
    #[serde(rename = "hints")]
    pub hints: Vec<crate::models::HintDto>,
}

impl TaskContentMissingCodeDto {
    pub fn new(
        kind: crate::models::TaskContentKindDto,
        content: String,
        correct_code: ::std::collections::HashMap<String, String>,
        hints: Vec<crate::models::HintDto>,
    ) -> TaskContentMissingCodeDto {
        TaskContentMissingCodeDto {
            kind,
            content,
            correct_code,
            hints,
        }
    }
}
