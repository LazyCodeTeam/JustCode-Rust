use std::{
    env::temp_dir,
    fs,
    path::{Path, PathBuf},
};

use common_domain::error::{Error, Result};

macro_rules! new_lang {
    ($lang:literal, $code: literal => $com:literal $($args1:literal)* {} $($args2:literal)*) => {
        paste::paste! {
            pub async fn [<create_ $lang _project>](target: &Path) -> Result<PathBuf> {
                let path = [<$lang _base_project_path>]();
                copy_project(&path, target)?;

                Ok(target.join($code))
            }
            pub fn [<$lang _base_project_path>]() -> PathBuf {
                base_projects_path().join([<$lang _base_project_name>]())
            }

            pub fn [<$lang _base_project_name>]() -> String {
                format!("{}_project", $lang)
            }

            pub fn [<create_base_ $lang _project>]() -> Result<()> {
                std::fs::remove_dir_all([<$lang _base_project_path>]()).ok();
                std::fs::create_dir_all(base_projects_path()).ok();
                let result = std::process::Command::new($com)
                    .current_dir(base_projects_path())
                    .args([$($args1),*, [<$lang _base_project_name>]().as_str(), $($args2),*])
                    .status()
                    .map_err(|e| Error::unknown(e.to_string()))?;

                let code_dir = [<$lang _base_project_path>]().join($code);
                std::fs::remove_dir_all(&code_dir).ok();
                std::fs::create_dir_all(&code_dir).ok();

                if result.success() {
                    Ok(())
                } else {
                    Err(Error::unknown(format!("Failed to create base {} project", $lang)))

                }
            }
        }
    };
}

pub fn base_projects_path() -> PathBuf {
    temp_dir().join("lazycode")
}

fn copy_project(from: &Path, to: &Path) -> Result<()> {
    fs::create_dir_all(to)
        .map_err(|e| Error::unknown(format!("Failed to copy base project to {to:?} ({e:?})",)))?;

    let subdirs = fs::read_dir(from)
        .map_err(|e| Error::unknown(format!("Failed to read subdirs of {from:?} ({e:?})")))?;
    for entry in subdirs {
        let entry =
            entry.map_err(|e| Error::unknown(format!("Failed to read data of file ({e:?})")))?;
        let file_type = entry.file_type().map_err(|e| {
            Error::unknown(format!("Failed to read file type of {entry:?} ({e:?})"))
        })?;

        if file_type.is_dir() {
            copy_project(&entry.path(), &to.join(entry.file_name()))?;
        } else {
            let from = entry.path();
            let to = to.join(entry.file_name());
            fs::copy(&from, &to).map_err(|e| {
                Error::unknown(format!("Failed to copy from {from:?} to {to:?} ({e:?})"))
            })?;
        }
    }

    Ok(())
}

new_lang!("dart", "bin" => "dart" "create" {});
