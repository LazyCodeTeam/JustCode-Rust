use bucket_domain::model::presigned_url::PresignedUrl;
use gen::models::PresignedUrlDto;

use crate::MapFrom;

impl MapFrom<PresignedUrl> for PresignedUrlDto {
    fn map_from(model: PresignedUrl) -> Self {
        Self {
            presigned_url: model.presigned_url,
            url: model.url,
            valid_until: model.valid_until.to_rfc3339(),
            headers: model.headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::Utc;

    use super::*;

    #[test]
    fn test_from_presigned_url() {
        let presigned_url = PresignedUrl {
            presigned_url: "https://example.com".to_string(),
            url: "https://example2.com".to_string(),
            valid_until: Utc::now(),
            headers: HashMap::from([("key".to_string(), "value".to_string())]),
        };

        let presigned_url_dto = PresignedUrlDto::map_from(presigned_url.clone());

        assert_eq!(presigned_url_dto.presigned_url, presigned_url.presigned_url);
        assert_eq!(presigned_url_dto.url, presigned_url.url);
        assert_eq!(
            presigned_url_dto.valid_until,
            presigned_url.valid_until.to_rfc3339()
        );
        assert_eq!(presigned_url_dto.headers, presigned_url.headers);
    }
}
