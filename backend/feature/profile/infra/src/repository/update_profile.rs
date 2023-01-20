use common_domain::error::Result;
use profile_domain::model::profile::Profile;

use crate::dto::put_profile_dto::PutProfileDto;

use super::save_profile::save_serialized_profile;

pub async fn update_profile(profile: Profile) -> Result<()> {
    let dto = PutProfileDto::from(profile);

    save_serialized_profile(dto).await
}
