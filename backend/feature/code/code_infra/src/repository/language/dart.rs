use std::path::{Path, PathBuf};

use code_domain::model::{diagnostic_result::DocumentDiagnostics, raw_message::RawMessage};
use common_domain::error::Result;

const BASE_PROJECT_NAME: &str = "dart_base_project";

pub async fn build(path: &Path) -> Result<PathBuf> {
    crate::repository::build::build(
        path,
        "dart",
        &[
            "compile",
            "js",
            "-O4",
            "-o",
            "build/out.js",
            "--no-source-maps",
            "lib/main.dart",
        ],
    )
    .await
    .map(|_| path.join("build").join("out.js"))
}

pub async fn get_version() -> Result<String> {
    crate::repository::version::get_version("dart", &["--version"]).await
}

pub async fn create_project(path: &Path) -> Result<PathBuf> {
    crate::repository::create_project::create_project(path, BASE_PROJECT_NAME, "lib").await
}

pub async fn create_base_project() -> Result<()> {
    let project_path =
        crate::repository::create_project::clone_base_project(BASE_PROJECT_NAME).await?;

    crate::repository::create_project::init_base_project(&project_path, "dart", &["pub", "get"])
        .await
}

pub async fn format(path: &Path) -> Result<()> {
    crate::repository::format::format(path, "dart", &["format", "."]).await
}

pub async fn raw_analyze(path: &Path) -> Result<RawMessage> {
    crate::repository::raw_analyze::raw_analyze(path, "dart", &["analyze", "."]).await
}

pub async fn analyze(path: &Path) -> Result<Vec<DocumentDiagnostics>> {
    crate::repository::analyze::analyze::<crate::dto::dart_diagnostic::DiagnosticResultDto>(
        path,
        "lib",
        "dart",
        &["analyze", "--format", "json", "."],
    )
    .await
}
