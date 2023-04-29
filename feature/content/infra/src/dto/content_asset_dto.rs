use chrono::{DateTime, Utc};
use content_domain::model::{
    content_asset::ContentAsset, content_asset_creation_data::ContentAssetCreationData,
};
use serde::{Deserialize, Serialize};

use crate::{MapFrom, CONTENT_ASSET_PK};

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

impl MapFrom<ContentAssetCreationData> for ContentAssetDto {
    fn map_from(model: ContentAssetCreationData) -> Self {
        let now = Utc::now();
        Self {
            pk: CONTENT_ASSET_PK.to_owned(),
            id: model.id,
            content_type: model.content_type,
            url: model.url,
            created_at: now,
        }
    }
}

impl MapFrom<ContentAssetDto> for ContentAsset {
    fn map_from(dto: ContentAssetDto) -> Self {
        Self {
            id: dto.id,
            content_type: dto.content_type,
            url: dto.url,
            created_at: dto.created_at,
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
        let content_asset_dto = ContentAssetDto::map_from(content_asset_creation_data);
        let after = Utc::now();

        assert_eq!(content_asset_dto.pk, CONTENT_ASSET_PK);
        assert_eq!(content_asset_dto.id, "id");
        assert_eq!(content_asset_dto.content_type, "content_type");
        assert_eq!(content_asset_dto.url, "url");
        assert!(content_asset_dto.created_at >= before);
        assert!(content_asset_dto.created_at <= after);
    }

    #[test]
    fn from_content_asset_dto() {
        let content_asset_dto = ContentAssetDto {
            pk: CONTENT_ASSET_PK.to_owned(),
            id: "id".to_owned(),
            content_type: "content_type".to_owned(),
            url: "url".to_owned(),
            created_at: Utc::now(),
        };

        let content_asset = ContentAsset::map_from(content_asset_dto.clone());

        assert_eq!(content_asset.id, content_asset_dto.id);
        assert_eq!(content_asset.content_type, content_asset_dto.content_type);
        assert_eq!(content_asset.url, content_asset_dto.url);
        assert_eq!(content_asset.created_at, content_asset_dto.created_at);
    }
}
