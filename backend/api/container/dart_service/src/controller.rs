use std::future;

use axum::http::StatusCode;
use axum::response::Response;
use axum::{response::IntoResponse, Json};
use code_domain::model::code_file::CodeFile;
use code_infra::format_code::format_dart;
use code_infra::read_files::read_files;
use code_infra::{
    create_project::create_dart_project, raw_analyze_code::analyze_dart, save_files::save_files,
};
use common_api::dto::axum_error_dto::ErrorResponseDto;
use common_api::dto::code::{file_dto::FileDto, raw_message_dto::RawMessageDto};
use common_infra::tmp::TmpDir;
use futures::TryFutureExt;
use use_case::format_code::format_code;
use use_case::raw_code_analyze::raw_code_analyze;

pub async fn format(Json(files): Json<Vec<FileDto>>) -> Result<Response, ErrorResponseDto> {
    let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();
    future::ready(TmpDir::new())
        .and_then(|tmp_dir| {
            format_code(
                tmp_dir,
                create_dart_project,
                save_files,
                format_dart,
                read_files,
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

pub async fn analyze_raw(Json(files): Json<Vec<FileDto>>) -> Result<Response, ErrorResponseDto> {
    let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();

    future::ready(TmpDir::new())
        .and_then(|tmp_dir| {
            raw_code_analyze(
                tmp_dir,
                create_dart_project,
                save_files,
                analyze_dart,
                &files,
            )
        })
        .await
        .map_err(ErrorResponseDto::from)
        .map(RawMessageDto::from)
        .map(|dto| (StatusCode::OK, Json(dto)).into_response())
}

pub async fn analyze(Json(files): Json<Vec<FileDto>>) -> Result<Response, ErrorResponseDto> {
    let files: Vec<CodeFile> = files.into_iter().map(CodeFile::from).collect();

    future::ready(TmpDir::new())
        .and_then(|tmp_dir| {
            raw_code_analyze(
                tmp_dir,
                create_dart_project,
                save_files,
                analyze_dart,
                &files,
            )
        })
        .await
        .map_err(ErrorResponseDto::from)
        .map(RawMessageDto::from)
        .map(|dto| (StatusCode::OK, Json(dto)).into_response())
}
