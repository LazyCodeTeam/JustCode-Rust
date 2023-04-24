use common_domain::error::Result;
use profile_domain::model::profile::Profile;

use crate::{dto::profile_dto::ProfileDto, FromModel};

use super::save_profile::save_serialized_profile;

pub async fn update_profile(profile: Profile) -> Result<()> {
    let dto = ProfileDto::from_model(profile);

    save_serialized_profile(dto).await
}
