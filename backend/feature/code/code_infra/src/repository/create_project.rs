use std::{
    env::temp_dir,
    fs,
    path::{Path, PathBuf},
};

use common_domain::error::{Error, Result};

const REPO: &str = "https://github.com/LazyCodeTeam";

pub async fn init_base_project(path: &Path, command: &str, args: &[&str]) -> Result<()> {
    tokio::process::Command::new(command)
        .current_dir(path)
        .args(args)
        .status()
        .await
        .map_err(|e| Error::unknown(e.to_string()))?;

    std::fs::remove_dir_all(path.join(".git")).ok();
    Ok(())
}

pub async fn clone_base_project(repo_name: &str) -> Result<PathBuf> {
    let base_path = base_projects_path();
    let repo_path = base_path.join(repo_name);
    tokio::fs::remove_dir_all(&repo_path).await.ok();
    tokio::fs::create_dir_all(&base_path).await.ok();
    let result = tokio::process::Command::new("git")
        .current_dir(&base_path)
        .args(["clone", &format!("{}/{}.git", REPO, repo_name)])
        .status()
        .await
        .map_err(|e| Error::unknown(e.to_string()))?;

    if result.success() {
        Ok(repo_path)
    } else {
        Err(Error::unknown(format!("Failed to clone {}", repo_name)))
    }
}

pub async fn create_project(target: &Path, repo_name: &str, code_dir: &str) -> Result<PathBuf> {
    copy_project(&base_projects_path().join(repo_name), target)?;

    Ok(target.join(code_dir))
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
