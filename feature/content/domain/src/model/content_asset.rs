use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ContentAsset {
    pub id: String,
    pub content_type: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}
