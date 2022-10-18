use common_domain::error::{Error, Result};
use std::{path::Path, process::Command};

macro_rules! new_lang {
    ($i:literal => $com:literal $($args:literal)*) => {
        paste::paste! {
            pub async fn [<format_ $i>](path: &Path) -> Result<()> {
                Command::new($com)
                    .current_dir(path)
                    .args(&[$($args),*])
                    .status()
                    .map_err(|e| Error::unknown(format!("Failed to format project {e:?}")))?;

                Ok(())
            }
        }
    };
}

#[cfg(feature = "dart")]
new_lang!("dart" => "dart" "format" "." "--show" "none" "--summary" "none");

#[cfg(feature = "flutter")]
new_lang!("flutter" => "dart" "format" "." "--show" "none" "--summary" "none");
