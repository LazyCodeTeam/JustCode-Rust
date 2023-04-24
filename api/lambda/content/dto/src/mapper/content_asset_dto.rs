use crate::ContentAssetDto;
use content_domain::model::content_asset::ContentAsset;

use crate::FromModel;

impl FromModel<ContentAsset> for ContentAssetDto {
    fn from_model(model: ContentAsset) -> Self {
        Self {
            id: model.id,
            mime: model.content_type,
            url: model.url,
            created_at: model.created_at.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_model() {
        let asset = ContentAsset {
            id: "id".to_string(),
            content_type: "content_type".to_string(),
            url: "url".to_string(),
            created_at: chrono::Utc::now(),
        };

        let dto = ContentAssetDto::from_model(asset.clone());

        assert_eq!(dto.id, asset.id);
        assert_eq!(dto.mime, asset.content_type);
        assert_eq!(dto.url, asset.url);
        assert_eq!(dto.created_at, asset.created_at.to_rfc3339());
    }

    #[test]
    fn from_model_vec() {
        let assets = vec![
            ContentAsset {
                id: "id1".to_string(),
                content_type: "content_type1".to_string(),
                url: "url1".to_string(),
                created_at: chrono::Utc::now(),
            },
            ContentAsset {
                id: "id2".to_string(),
                content_type: "content_type2".to_string(),
                url: "url2".to_string(),
                created_at: chrono::Utc::now(),
            },
        ];

        let dtos = Vec::<ContentAssetDto>::from_model(assets.clone());

        assert_eq!(dtos.len(), assets.len());
        assert_eq!(dtos[0].id, assets[0].id);
        assert_eq!(dtos[0].mime, assets[0].content_type);
        assert_eq!(dtos[0].url, assets[0].url);
        assert_eq!(dtos[0].created_at, assets[0].created_at.to_rfc3339());
        assert_eq!(dtos[1].id, assets[1].id);
        assert_eq!(dtos[1].mime, assets[1].content_type);
        assert_eq!(dtos[1].url, assets[1].url);
        assert_eq!(dtos[1].created_at, assets[1].created_at.to_rfc3339());
    }
}
