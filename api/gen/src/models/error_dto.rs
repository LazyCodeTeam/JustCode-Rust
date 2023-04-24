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
pub struct ErrorDto {
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "args")]
    pub args: ::std::collections::HashMap<String, String>,
}

impl ErrorDto {
    pub fn new(
        message: String,
        code: String,
        args: ::std::collections::HashMap<String, String>,
    ) -> ErrorDto {
        ErrorDto {
            message,
            code,
            args,
        }
    }
}