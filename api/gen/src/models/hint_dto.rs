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
pub struct HintDto {
    #[serde(rename = "content")]
    pub content: String,
}

impl HintDto {
    pub fn new(content: String) -> HintDto {
        HintDto { content }
    }
}
