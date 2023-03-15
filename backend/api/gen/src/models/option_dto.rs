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
pub struct OptionDto {
    #[serde(rename = "id")]
    pub id: u16,
    #[serde(rename = "content")]
    pub content: String,
}

impl OptionDto {
    pub fn new(id: u16, content: String) -> OptionDto {
        OptionDto { id, content }
    }
}
