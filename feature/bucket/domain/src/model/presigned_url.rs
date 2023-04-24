use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct PresignedUrl {
    pub presigned_url: String,
    pub url: String,
    pub valid_until: DateTime<Utc>,
    pub headers: HashMap<String, String>,
}
