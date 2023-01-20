use chrono::{DateTime, Utc};
use profile_domain::model::create_profile_params::CreateProfileParams;
use serde::Serialize;

use crate::{PROFILE_ID_PREFIX, PROFILE_SORT_KEY};

#[derive(Serialize, PartialEq, Eq, Debug)]
pub struct PutProfileDto {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub name: String,
    pub email: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<CreateProfileParams> for PutProfileDto {
    fn from(params: CreateProfileParams) -> Self {
        Self {
            pk: format!("{}{}", PROFILE_ID_PREFIX, params.id),
            sk: PROFILE_SORT_KEY.to_owned(),
            name: params.name,
            email: params.email,
            updated_at: Utc::now(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let params = CreateProfileParams {
            id: "id".to_owned(),
            name: "name".to_owned(),
            email: "email".to_owned(),
        };
        let dto = PutProfileDto::from(params);
        assert_eq!(
            dto,
            PutProfileDto {
                pk: format!("{}{}", PROFILE_ID_PREFIX, "id"),
                sk: PROFILE_SORT_KEY.to_owned(),
                name: "name".to_owned(),
                email: "email".to_owned(),
                updated_at: dto.updated_at,
                created_at: dto.created_at,
            }
        );
    }
}
