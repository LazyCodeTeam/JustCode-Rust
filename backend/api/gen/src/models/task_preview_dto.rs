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
pub struct TaskPreviewDto {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "is_available")]
    pub is_available: bool,
}

impl TaskPreviewDto {
    pub fn new(id: String, title: String, is_available: bool) -> TaskPreviewDto {
        TaskPreviewDto {
            id,
            title,
            is_available,
        }
    }
}
