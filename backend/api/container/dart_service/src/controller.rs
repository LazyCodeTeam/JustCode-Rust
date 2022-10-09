use std::future;

use actix_web::{post, web::Json, HttpResponse, Responder};

use code_domain::model::code_file::CodeFile;
use code_infra::format_code::format_dart;
use code_infra::read_files::read_files;
use code_infra::{
    create_project::create_dart_project, raw_analyze_code::analyze_dart, save_files::save_files,
};
use common_api::dto::code::{file_dto::FileDto, raw_message_dto::RawMessageDto};
use common_api::handle_error;
use common_infra::tmp::TmpDir;
use futures::TryFutureExt;
use use_case::format_code::format_code;
use use_case::raw_code_analyze::raw_code_analyze;

#[post("/format")]
async fn format(files: Json<Vec<FileDto>>) -> impl Responder {
    let files: Vec<CodeFile> = files.into_inner().into_iter().map(CodeFile::from).collect();
    let result = future::ready(TmpDir::new())
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
        .await;

    let dtos: Vec<FileDto> = handle_error!(result)
        .into_iter()
        .map(FileDto::from)
        .collect();

    HttpResponse::Ok().json(dtos)
}

#[post("/analyze/raw")]
async fn analyze_raw(files: Json<Vec<FileDto>>) -> impl Responder {
    let files: Vec<CodeFile> = files.into_inner().into_iter().map(CodeFile::from).collect();

    let result = future::ready(TmpDir::new())
        .and_then(|tmp_dir| {
            raw_code_analyze(
                tmp_dir,
                create_dart_project,
                save_files,
                analyze_dart,
                &files,
            )
        })
        .await;

    let dto = RawMessageDto::from(handle_error!(result));

    HttpResponse::Ok().json(dto)
}
