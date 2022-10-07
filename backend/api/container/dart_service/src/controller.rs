use std::future;

use actix_web::{post, web::Json, HttpResponse, Responder};

use code_domain::model::code_file::CodeFile;
use code_infra::{
    create_project::create_dart_project, raw_analyze_code::analyze_dart, save_files::save_files,
};
use common_api::dto::code::{file_dto::FileDto, raw_message_dto::RawMessageDto};
use common_infra::tmp::TmpDir;
use futures::TryFutureExt;
use use_case::raw_code_analyze::raw_code_analyze;

#[post("/format")]
async fn format(files: Json<Vec<FileDto>>) -> impl Responder {
    HttpResponse::Ok().json(files)
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
    println!("Result: {result:?}");

    let dto = RawMessageDto {
        success: true,
        message: "Raw dart analyze result".to_owned(),
    };

    HttpResponse::Ok().json(dto)
}
