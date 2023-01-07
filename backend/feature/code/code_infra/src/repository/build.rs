use std::{collections::HashMap, path::Path};

use common_domain::error::{Error, ErrorOutput, ErrorType, Result};
use tokio::process::Command;

pub(crate) async fn build(path: &Path, command: &str, args: &[&str]) -> Result<()> {
    let out = Command::new(command)
        .current_dir(path)
        .args(args)
        .output()
        .await
        .map_err(|e| Error::unknown(format!("Failed to build project {e:?}")))?;

    if out.status.success() {
        Ok(())
    } else {
        let message = String::from_utf8_lossy(&out.stdout).to_string();
        Err(Error {
            debug_message: format!("Failed to build {command}: {message}"),
            error_type: ErrorType::InvalidInput,
            output: Box::new(ErrorOutput {
                message: "Build failed".to_owned(),
                code: "build_failed".to_owned(),
                args: HashMap::from([("output".to_owned(), message)]),
            }),
        })
    }
}
