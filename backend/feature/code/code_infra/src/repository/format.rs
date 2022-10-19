use std::process::Command;

use common_domain::error::{Error, Result};

pub async fn format(path: &std::path::Path, command: &str, args: &[&str]) -> Result<()> {
    Command::new(command)
        .current_dir(path)
        .args(args)
        .status()
        .map_err(|e| Error::unknown(format!("Failed to format project {e:?}")))?;

    Ok(())
}
