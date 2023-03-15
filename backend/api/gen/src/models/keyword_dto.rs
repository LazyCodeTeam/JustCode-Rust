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
pub struct KeywordDto {
    #[serde(rename = "id")]
    pub id: u16,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "modifiers")]
    pub modifiers: Vec<crate::models::KeywordModifierDto>,
}

impl KeywordDto {
    pub fn new(
        id: u16,
        content: String,
        modifiers: Vec<crate::models::KeywordModifierDto>,
    ) -> KeywordDto {
        KeywordDto {
            id,
            content,
            modifiers,
        }
    }
}
