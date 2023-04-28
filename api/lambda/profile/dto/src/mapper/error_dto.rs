use common_api::lambda::lambda_error::LambdaError;
use use_case::profile::{
    delete_profile::DeleteProfileError, get_profile_by_id::GetProfileByIdError,
    on_avatars_created::OnAvatarsCreatedError, request_avatar_upload::RequestAvatarUploadError,
    set_push_data::SetPushDataError, update_profile::UpdateProfileError,
};

use crate::MapFrom;

impl MapFrom<DeleteProfileError> for LambdaError {
    fn map_from(value: DeleteProfileError) -> Self {
        match value {
            DeleteProfileError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<GetProfileByIdError> for LambdaError {
    fn map_from(value: GetProfileByIdError) -> Self {
        match value {
            GetProfileByIdError::NotFound => LambdaError::not_found(),
            GetProfileByIdError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<OnAvatarsCreatedError> for LambdaError {
    fn map_from(value: OnAvatarsCreatedError) -> Self {
        match value {
            OnAvatarsCreatedError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<SetPushDataError> for LambdaError {
    fn map_from(value: SetPushDataError) -> Self {
        match value {
            SetPushDataError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<UpdateProfileError> for LambdaError {
    fn map_from(value: UpdateProfileError) -> Self {
        match value {
            UpdateProfileError::Infra { .. } => LambdaError::internal_server_error(),
            UpdateProfileError::NotFound => LambdaError::not_found(),
        }
    }
}

impl MapFrom<RequestAvatarUploadError> for LambdaError {
    fn map_from(value: RequestAvatarUploadError) -> Self {
        match value {
            RequestAvatarUploadError::Infra { .. } => LambdaError::internal_server_error(),
            RequestAvatarUploadError::NotFound => LambdaError::not_found(),
        }
    }
}

#[cfg(test)]
mod test {
    use common_domain::error::Error;

    use super::*;

    fn common_result() -> Result<(), Error> {
        snafu::whatever!("error")
    }

    #[test]
    fn from_request_avatar_upload_error_not_found() {
        let error = RequestAvatarUploadError::NotFound;
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found());
    }

    #[test]
    fn from_request_avatar_upload_error_infra() {
        let error = RequestAvatarUploadError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error());
    }

    #[test]
    fn from_update_profile_error_not_found() {
        let error = UpdateProfileError::NotFound;
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found());
    }

    #[test]
    fn from_update_profile_error_infra() {
        let error = UpdateProfileError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error());
    }

    #[test]
    fn from_set_push_data_error_infra() {
        let error = SetPushDataError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error());
    }

    #[test]
    fn from_on_avatars_created_error_infra() {
        let error = OnAvatarsCreatedError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error());
    }

    #[test]
    fn from_get_profile_by_id_not_found() {
        let error = GetProfileByIdError::NotFound;
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found());
    }

    #[test]
    fn from_get_profile_by_id_error_infra() {
        let error = GetProfileByIdError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error());
    }

    #[test]
    fn from_delete_profile_error_infra() {
        let error = DeleteProfileError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error());
    }
}
