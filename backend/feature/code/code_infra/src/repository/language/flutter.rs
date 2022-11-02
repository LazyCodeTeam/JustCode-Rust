use std::path::{Path, PathBuf};

use common_domain::error::Error;
use common_domain::error::ErrorDetails;
use common_domain::error::ErrorType;
use common_domain::error::Result;

// Implementation is same as dart
pub use crate::repository::dart::analyze;
pub use crate::repository::dart::format;
pub use crate::repository::dart::raw_analyze;

const BASE_PROJECT_NAME: &str = "flutter_base_project";

pub async fn build(path: &Path) -> Result<PathBuf> {
    crate::repository::build::build(path, "flutter", &["build", "web", "--no-pub", "--release"])
        .await
        .map(|_| path.join("build").join("web"))
}

pub async fn get_version() -> Result<String> {
    crate::repository::version::get_version("flutter", &["--version"]).await
}

pub async fn create_project(path: &Path) -> Result<PathBuf> {
    crate::repository::create_project::create_project(path, BASE_PROJECT_NAME, "lib").await
}

pub async fn read_js(_path: &Path) -> Result<String> {
    Err(Error::builder()
        .set_error_type(ErrorType::InvalidInput)
        .set_debug_message("`read_js` is not supported for flutter".to_owned())
        .set_details(ErrorDetails {
            message: "Compilation to js is not supported".to_owned(),
            code: "js_compilation_not_supported".to_owned(),
            args: None,
        })
        .build())
}

pub async fn create_base_project() -> Result<()> {
    let project_path =
        crate::repository::create_project::clone_base_project(BASE_PROJECT_NAME).await?;

    crate::repository::create_project::init_base_project(&project_path, "flutter", &["pub", "get"])
        .await
}
