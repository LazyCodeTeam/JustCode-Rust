use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub struct TaskPreviewDto {
    pub id: String,
    pub title: String,
}
