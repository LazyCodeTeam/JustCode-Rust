use common_api::lambda::lambda_error::LambdaError;
use gen::models::ErrorDto;
use use_case::content::{
    answer::AnswerError, delete_content_assets::DeleteContentAssetsError,
    get_content_assets::GetContentAssetsError, get_public_sections::GetPublicSectionsError,
    get_public_tasks::GetPublicTasksError, get_public_technologies::GetPublicTechnologiesError,
    get_sections::GetSectionsError, get_tasks::GetTasksError, load_content::LoadContentError,
    request_assets_upload::RequestAssetsUploadError,
};

use crate::MapFrom;

impl MapFrom<AnswerError> for LambdaError {
    fn map_from(value: AnswerError) -> Self {
        match value {
            AnswerError::TaskNotFound { .. } => LambdaError::not_found(),
            AnswerError::InvalidAnswerType { .. } => LambdaError {
                code: http::StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    code: "invalid_answer_type".to_string(),
                    message: "Invalid answer type".to_string(),
                    ..Default::default()
                },
            },
            AnswerError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<DeleteContentAssetsError> for LambdaError {
    fn map_from(value: DeleteContentAssetsError) -> Self {
        match value {
            DeleteContentAssetsError::SomeAssetsNotDeleted { failed } => LambdaError {
                code: http::StatusCode::INTERNAL_SERVER_ERROR,
                dto: ErrorDto {
                    code: "some_assets_not_deleted".to_string(),
                    message: format!("Some assets not deleted: {:?}", failed),
                    args: failed
                        .into_iter()
                        .enumerate()
                        .map(|(i, id)| (i.to_string(), id))
                        .collect(),
                },
            },
            DeleteContentAssetsError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<GetContentAssetsError> for LambdaError {
    fn map_from(value: GetContentAssetsError) -> Self {
        match value {
            GetContentAssetsError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<GetPublicSectionsError> for LambdaError {
    fn map_from(value: GetPublicSectionsError) -> Self {
        match value {
            GetPublicSectionsError::Infra { .. } => LambdaError::internal_server_error(),
            GetPublicSectionsError::NotFound { .. } => LambdaError::not_found(),
        }
    }
}

impl MapFrom<GetPublicTasksError> for LambdaError {
    fn map_from(value: GetPublicTasksError) -> Self {
        match value {
            GetPublicTasksError::Infra { .. } => LambdaError::internal_server_error(),
            GetPublicTasksError::NotFound { .. } => LambdaError::not_found(),
        }
    }
}

impl MapFrom<GetPublicTechnologiesError> for LambdaError {
    fn map_from(value: GetPublicTechnologiesError) -> Self {
        match value {
            GetPublicTechnologiesError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<GetTasksError> for LambdaError {
    fn map_from(value: GetTasksError) -> Self {
        match value {
            GetTasksError::Infra { .. } => LambdaError::internal_server_error(),
            GetTasksError::NotFound => LambdaError::not_found(),
        }
    }
}

impl MapFrom<LoadContentError> for LambdaError {
    fn map_from(value: LoadContentError) -> Self {
        match value {
            LoadContentError::NotModified => LambdaError::not_modified(),
            LoadContentError::TransactionAlreadyInProgress => LambdaError {
                code: http::StatusCode::CONFLICT,
                dto: ErrorDto {
                    code: "transaction_already_in_progress".to_string(),
                    message: "Transaction already in progress".to_string(),
                    ..Default::default()
                },
            },
            LoadContentError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<RequestAssetsUploadError> for LambdaError {
    fn map_from(value: RequestAssetsUploadError) -> Self {
        match value {
            RequestAssetsUploadError::Infra { .. } => LambdaError::internal_server_error(),
        }
    }
}

impl MapFrom<GetSectionsError> for LambdaError {
    fn map_from(value: GetSectionsError) -> Self {
        match value {
            GetSectionsError::Infra { .. } => LambdaError::internal_server_error(),
            GetSectionsError::NotFound => LambdaError::not_found(),
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
    fn load_content_error_not_modified() {
        let error = LoadContentError::NotModified;
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_modified())
    }

    #[test]
    fn load_content_error_transaction_already_in_progress() {
        let error = LoadContentError::TransactionAlreadyInProgress;
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(
            lambda_error,
            LambdaError {
                code: http::StatusCode::CONFLICT,
                dto: ErrorDto {
                    code: "transaction_already_in_progress".to_string(),
                    message: "Transaction already in progress".to_string(),
                    ..Default::default()
                },
            }
        )
    }

    #[test]
    fn load_content_error_infra() {
        let error = LoadContentError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn get_tasks_error_not_found() {
        let error = GetTasksError::NotFound;
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found())
    }

    #[test]
    fn get_tasks_error_infra() {
        let error = GetTasksError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn get_public_technologies_error_infra() {
        let error = GetPublicTechnologiesError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn get_public_tasks_error_not_found() {
        let error = GetPublicTasksError::NotFound {
            section_id: "section_id".to_string(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found())
    }

    #[test]
    fn get_public_tasks_error_infra() {
        let error = GetPublicTasksError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn get_public_sections_error_not_found() {
        let error = GetPublicSectionsError::NotFound {
            technology_id: "technology_id".to_string(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found())
    }

    #[test]
    fn get_public_sections_error_infra() {
        let error = GetPublicSectionsError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn get_content_assets_error_infra() {
        let error = GetContentAssetsError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn delete_content_assets_error_some_assets_not_deleted() {
        let error = DeleteContentAssetsError::SomeAssetsNotDeleted {
            failed: vec!["id1".to_string(), "id2".to_string()],
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(
            lambda_error,
            LambdaError {
                code: http::StatusCode::INTERNAL_SERVER_ERROR,
                dto: ErrorDto {
                    code: "some_assets_not_deleted".to_string(),
                    message: "Some assets not deleted: [\"id1\", \"id2\"]".to_string(),
                    args: vec![
                        ("0".to_string(), "id1".to_string()),
                        ("1".to_string(), "id2".to_string())
                    ]
                    .into_iter()
                    .collect(),
                },
            }
        )
    }

    #[test]
    fn delete_content_assets_error_infra() {
        let error = DeleteContentAssetsError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }

    #[test]
    fn answer_error_task_not_found() {
        let error = AnswerError::TaskNotFound {
            task_id: "task_id".to_string(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::not_found())
    }

    #[test]
    fn answer_error_invalid_answer_type() {
        let error = AnswerError::InvalidAnswerType {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(
            lambda_error,
            LambdaError {
                code: http::StatusCode::BAD_REQUEST,
                dto: ErrorDto {
                    code: "invalid_answer_type".to_string(),
                    message: "Invalid answer type".to_string(),
                    ..Default::default()
                },
            }
        )
    }

    #[test]
    fn answer_error_infra() {
        let error = AnswerError::Infra {
            source: common_result().unwrap_err(),
        };
        let lambda_error = LambdaError::map_from(error);
        assert_eq!(lambda_error, LambdaError::internal_server_error())
    }
}
