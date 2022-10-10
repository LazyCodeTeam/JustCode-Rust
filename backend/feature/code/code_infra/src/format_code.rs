use common_domain::error::{Error, Result};
use std::{path::Path, process::Command};

macro_rules! new_lang {
    ($i:literal => $com:literal $($args:literal)*) => {
        paste::paste! {
            pub async fn [<format_ $i>](path: &Path) -> Result<()> {
                format(path, $com, &[$($args),*]).await
            }
        }
    };
}

async fn format(path: &Path, command: &str, args: &[&str]) -> Result<()> {
    Command::new(command)
        .current_dir(path)
        .args(args)
        .status()
        .map_err(|e| Error::unknown(format!("Failed to format project {e:?}")))?;

    Ok(())
}

new_lang!("dart" => "dart" "format" ".");
