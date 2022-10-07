use std::{env::temp_dir, path::PathBuf};

use common_domain::error::{Error, Result};
use fs_extra::dir::CopyOptions;

macro_rules! new_lang {
    ($project:literal, $base:literal => $pn:ident, $bp:ident, $cp: ident) => {
        pub async fn $cp(target: &PathBuf) -> Result<PathBuf> {
            let path = $bp();
            copy_project(&path, target).await?;

            Ok(path.join($base))
        }
        pub fn $bp() -> PathBuf {
            base_projects_path().join($project)
        }

        pub fn $pn() -> String {
            $project.to_owned()
        }
    };
}

pub fn base_projects_path() -> PathBuf {
    temp_dir().join("tessar").join("base")
}

async fn copy_project(from: &PathBuf, to: &PathBuf) -> Result<()> {
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.copy_inside = true;

    fs_extra::dir::copy(from, to, &options).map_err(|e| Error::unknown(&e.to_string()))?;
    Ok(())
}

new_lang!("dart_project", "bin" => dart_project_name, base_dart_project_path, create_dart_project);
