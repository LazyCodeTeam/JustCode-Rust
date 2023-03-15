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
pub struct PublicTaskAvailableDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "difficulty")]
    pub difficulty: u8,
    #[serde(rename = "kind")]
    pub kind: crate::models::PublicTaskKindDto,
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<Box<crate::models::TaskContentDto>>,
}

impl PublicTaskAvailableDto {
    pub fn new(
        id: String,
        title: String,
        difficulty: u8,
        kind: crate::models::PublicTaskKindDto,
    ) -> PublicTaskAvailableDto {
        PublicTaskAvailableDto {
            id,
            title,
            difficulty,
            kind,
            content: None,
        }
    }
}
