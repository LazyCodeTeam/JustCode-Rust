use chrono::{DateTime, Utc};
use content_domain::model::content_asset_creation_data::ContentAssetCreationData;
use serde::{Deserialize, Serialize};

use crate::CONTENT_ASSET_PK;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContentAssetDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub id: String,
    pub content_type: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl From<ContentAssetCreationData> for ContentAssetDto {
    fn from(value: ContentAssetCreationData) -> Self {
        Self {
            pk: CONTENT_ASSET_PK.to_owned(),
            id: value.id,
            content_type: value.content_type,
            url: value.url,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use super::*;

    #[test]
    fn from_content_asset_creation_data() {
        let content_asset_creation_data = ContentAssetCreationData {
            id: "id".to_owned(),
            content_type: "content_type".to_owned(),
            url: "url".to_owned(),
        };

        let before = Utc::now();
        let content_asset_dto = ContentAssetDto::from(content_asset_creation_data);
        let after = Utc::now();

        assert_eq!(content_asset_dto.pk, CONTENT_ASSET_PK);
        assert_eq!(content_asset_dto.id, "id");
        assert_eq!(content_asset_dto.content_type, "content_type");
        assert_eq!(content_asset_dto.url, "url");
        assert!(content_asset_dto.created_at >= before);
        assert!(content_asset_dto.created_at <= after);
    }
}
