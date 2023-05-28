use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct PersonalizedTaskPreview {
    pub id: String,
    pub title: String,
    pub for_anonymous: bool,
    pub done_at: Option<DateTime<Utc>>,
}
