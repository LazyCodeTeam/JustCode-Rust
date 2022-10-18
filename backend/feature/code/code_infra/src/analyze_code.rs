use code_domain::model::diagnostic_result::DocumentDiagnostics;
use common_domain::error::{Error, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{path::Path, process::Command};

macro_rules! new_lang {
    ($i:literal, $files:literal, $dto:ident => $com:literal $($args:literal)*) => {
        paste::paste! {
            pub async fn [<analyze_ $i>](path: &Path) -> Result<Vec<DocumentDiagnostics>> {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"\{.*\}").expect("Failed to compile regex");
                }
                let result = Command::new($com)
                    .current_dir(path)
                    .args(&[$($args),*])
                    .output()
                    .map_err(|e| Error::unknown(format!("Failed to execute analyze command: {e:?}",)))?;

                let message = String::from_utf8_lossy(&result.stdout).to_string();
                let path = path
                    .join($files)
                    .to_owned()
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

                        let diagnostic: crate::dto::$dto::DiagnosticResultDto = serde_json::from_str(&message)
                            .map_err(|e| Error::unknown(format!("Failed to parse diagnostics: {e:?}")))?;

                        Ok(diagnostic.into())
                    }
                    None => Ok(Vec::new()),
                }
            }
        }
    };
}

#[cfg(feature = "dart")]
new_lang!("dart", "lib", dart_diagnostic => "dart" "analyze" "." "--format" "json");

#[cfg(feature = "flutter")]
new_lang!("flutter", "lib", dart_diagnostic => "dart" "analyze" "." "--format" "json");
