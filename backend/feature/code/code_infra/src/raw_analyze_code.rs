use code_domain::model::raw_message::RawMessage;
use common_domain::error::{Error, Result};
use std::{path::PathBuf, process::Command};

macro_rules! new_lang {
    ($i:ident => $com:literal, $($args:literal),*) => {
        pub async fn $i(path: &PathBuf) -> Result<RawMessage> {
            analyze(path, $com, &[$($args),*]).await
        }
    };
}

async fn analyze(path: &PathBuf, command: &str, args: &[&str]) -> Result<RawMessage> {
    let result = Command::new(command)
        .current_dir(path)
        .args(args)
        .output()
        .map_err(|e| Error::unknown(&e.to_string()))?;

    let message = if result.status.success() {
        String::from_utf8_lossy(&result.stdout).to_string()
    } else {
        String::from_utf8_lossy(&result.stderr).to_string()
    };

    Ok(RawMessage {
        success: result.status.success(),
        message,
    })
}

new_lang!(analyze_dart => "dart", "analyze", ".");
