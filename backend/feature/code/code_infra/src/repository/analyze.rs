use code_domain::model::diagnostic_result::DocumentDiagnostics;
use common_domain::error::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::{path::Path, process::Command};

pub(crate) async fn analyze<'a, T>(
    path: &'a Path,
    files_dir: &'a str,
    command: &'a str,
    args: &'a [&'a str],
) -> Result<Vec<DocumentDiagnostics>>
where
    for<'b> T: Deserialize<'b> + Into<Vec<DocumentDiagnostics>>,
{
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{.*\}").expect("Failed to compile regex");
    }
    let result = Command::new(command)
        .current_dir(path)
        .args(args)
        .output()
        .map_err(|e| Error::unknown(format!("Failed to execute analyze command: {e:?}",)))?;

    let message = String::from_utf8_lossy(&result.stdout).to_string();
    let path = path
        .join(files_dir)
        .into_os_string()
        .into_string()
        .map(|s| format!("{s}/"))
        .map_err(|e| Error::unknown(format!("Failed to parse path to String: {e:?}")))?;
    let message = message.replace(&path, "");
    let capture = RE.captures(&message);

    match capture {
        Some(capture) => {
            let message = capture
                .get(0)
                .ok_or_else(|| {
                    Error::unknown(
                        "Group 0 of regex is empty!!! This should not happend".to_owned(),
                    )
                })?
                .as_str()
                .to_owned()
                .replace("\\\"", "\"");

            let diagnostic: T = serde_json::from_str(&message)
                .map_err(|e| Error::unknown(format!("Failed to parse diagnostics: {e:?}")))?;

            Ok(diagnostic.into())
        }
        None => Ok(Vec::new()),
    }
}
