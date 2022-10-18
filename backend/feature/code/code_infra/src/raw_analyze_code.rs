use code_domain::model::raw_message::RawMessage;
use common_domain::error::{Error, Result};
use std::{path::Path, process::Command};

macro_rules! new_lang {
    ($i:literal => $com:literal $($args:literal)*) => {
        paste::paste! {
            pub async fn [<raw_analyze_ $i>](path: &Path) -> Result<RawMessage> {
                let result = Command::new($com)
                    .current_dir(path)
                    .args(&[$($args),*])
                    .output()
                    .map_err(|e| Error::unknown(e.to_string()))?;

                let message = String::from_utf8_lossy(&result.stdout).to_string();

                Ok(RawMessage {
                    success: result.status.success(),
                    message,
                })
            }
        }
    };
}

#[cfg(feature = "dart")]
new_lang!("dart" => "dart" "analyze" ".");

#[cfg(feature = "flutter")]
new_lang!("flutter" => "dart" "analyze" ".");
