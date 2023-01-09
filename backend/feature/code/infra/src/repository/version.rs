use tokio::process::Command;

use common_domain::error::{Error, Result};

pub async fn get_version(command: &str, args: &[&str]) -> Result<String> {
    let out = Command::new(command)
        .args(args)
        .output()
        .await
        .map_err(|e| Error::unknown(format!("Failed to execute version command: {e:?}")))?;

    if out.status.success() {
        let message = String::from_utf8_lossy(&out.stdout).to_string();

        Ok(message)
    } else {
        let message = String::from_utf8_lossy(&out.stderr).to_string();

        Err(Error::unknown(format!(
            "Failed to get {command} version: {message}"
        )))
    }
}
