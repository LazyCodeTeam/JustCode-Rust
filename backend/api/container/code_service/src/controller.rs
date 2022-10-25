macro_rules! new_lang {
    ($lang:ident) => {
        pub mod $lang {
            use crate::dto::{file_dto::FileDto, raw_message_dto::RawMessageDto};
            use axum::http::StatusCode;
            use axum::response::Response;
            use axum::{response::IntoResponse, Json};
            use code_domain::model::code_file::CodeFile;
            use common_api::dto::axum_error_dto::ErrorResponseDto;
            use common_infra::tmp::TmpDir;
            use futures::TryFutureExt;
            use std::future;

            use code_infra::repository::$lang as lang_repo;

            new_lang!(format: lang_repo);
            new_lang!(analyze_raw: lang_repo);
            new_lang!(analyze: lang_repo);
            new_lang!(get_version: lang_repo);
            new_lang!(build: lang_repo);
        }
    };
    (format: $repo:path) => {
        pub async fn format(Json(files): Json<Vec<FileDto>>) -> Result<Response, ErrorResponseDto> {
            use $repo as repo;

            let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();
            future::ready(TmpDir::new())
                .and_then(|tmp_dir| {
                    use_case::format_code::format_code(
                        tmp_dir,
                        repo::create_project,
                        code_infra::repository::save_files,
                        repo::format,
                        code_infra::repository::read_files,
                        &files,
                    )
                })
                .await
                .map_err(ErrorResponseDto::from)
                .map(|files| {
                    (
                        StatusCode::OK,
                        Json(
                            files
                                .into_iter()
                                .map(FileDto::from)
                                .collect::<Vec<FileDto>>(),
                        ),
                    )
                        .into_response()
                })
        }
    };
    (analyze_raw: $repo:path) => {
        pub async fn analyze_raw(
            Json(files): Json<Vec<FileDto>>,
        ) -> Result<Response, ErrorResponseDto> {
            use $repo as repo;

            let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();

            future::ready(TmpDir::new())
                .and_then(|tmp_dir| {
                    use_case::raw_code_analyze::raw_code_analyze(
                        tmp_dir,
                        repo::create_project,
                        code_infra::repository::save_files,
                        repo::raw_analyze,
                        &files,
                    )
                })
                .await
                .map_err(ErrorResponseDto::from)
                .map(RawMessageDto::from)
                .map(|dto| (StatusCode::OK, Json(dto)).into_response())
        }
    };
    (analyze: $repo:path) => {
        pub async fn analyze(
            Json(files): Json<Vec<FileDto>>,
        ) -> Result<Response, ErrorResponseDto> {
            use crate::dto::diagnostic_result_dto::DocumentDiagnosticsDto;
            use $repo as repo;

            let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();

            future::ready(TmpDir::new())
                .and_then(|tmp_dir| {
                    use_case::analyze_code::analyze_code(
                        tmp_dir,
                        repo::create_project,
                        code_infra::repository::save_files,
                        repo::analyze,
                        &files,
                    )
                })
                .await
                .map_err(ErrorResponseDto::from)
                .map(|diagnostics| {
                    diagnostics
                        .into_iter()
                        .map(DocumentDiagnosticsDto::from)
                        .collect::<Vec<DocumentDiagnosticsDto>>()
                })
                .map(|dto| (StatusCode::OK, Json(dto)).into_response())
        }
    };
    (get_version: $repo:path) => {
        pub async fn get_version() -> Result<Response, ErrorResponseDto> {
            use crate::dto::version_response_dto::VersionResponseDto;

            use $repo as repo;

            use_case::get_lang_version::get_lang_version(repo::get_version)
                .await
                .map_err(ErrorResponseDto::from)
                .map(VersionResponseDto::from)
                .map(|dto| (StatusCode::OK, Json(dto)).into_response())
        }
    };
    (build: $repo:path) => {
        pub async fn build(Json(files): Json<Vec<FileDto>>) -> Result<Response, ErrorResponseDto> {
            use $repo as repo;

            let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();

            future::ready(TmpDir::new())
                .and_then(|tmp_dir| {
                    use_case::build_code::build_code(
                        tmp_dir,
                        repo::create_project,
                        code_infra::repository::save_files,
                        repo::build,
                        common_infra::repository::compress,
                        &files,
                    )
                })
                .await
                .map_err(ErrorResponseDto::from)
                .map(tokio_util::io::ReaderStream::new)
                .map(axum::body::StreamBody::new)
                .map(|stream| {
                    (
                        [(
                            axum::http::header::CONTENT_DISPOSITION,
                            "attachment; filename=\"out.zip\"",
                        )],
                        stream,
                    )
                        .into_response()
                })
        }
    };
}

#[cfg(feature = "dart")]
new_lang!(dart);

#[cfg(feature = "flutter")]
new_lang!(flutter);
