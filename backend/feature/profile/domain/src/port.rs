use crate::model::create_profile_params::CreateProfileParams;
use crate::model::profile::Profile;
use crate::model::push_data::PushData;
use common_domain::define_port;
use common_domain::error::Result;

define_port!(GetProfileById = Fn<'a>(id: &'a str) -> Result<Option<Profile>>);

define_port!(SaveProfile = Fn(params: CreateProfileParams) -> Result<()>);

define_port!(UpdateProfile = Fn(params: Profile) -> Result<()>);

define_port!(UpdateProfileAvatar = Fn<'a>(id: &'a str, url: Option<&'a str>) -> Result<()>);

define_port!(UpdatePushData = Fn<'a>(id: &'a str, data: &'a PushData) -> Result<()>);

define_port!(RemovePushData = Fn<'a>(id: &'a str) -> Result<()>);
