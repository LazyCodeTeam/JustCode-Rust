use serde::Deserialize;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum TaskContentDto {
    Lesson { content: String },
}

impl Validate for TaskContentDto {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}
