use code_domain::model::raw_message::RawMessage;
use common_domain::error::{Error, Result};
use std::path::Path;
use tokio::process::Command;

pub async fn raw_analyze(path: &Path, command: &str, args: &[&str]) -> Result<RawMessage> {
    let result = Command::new(command)
        .current_dir(path)
        .args(args)
        .output()
        .await
        .map_err(|e| Error::unknown(e.to_string()))?;

    let message = String::from_utf8_lossy(&result.stdout).to_string();

    Ok(RawMessage {
        success: result.status.success(),
        message,
    })
}
