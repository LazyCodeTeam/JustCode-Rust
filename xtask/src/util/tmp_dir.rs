use std::path::PathBuf;

use nanoid::nanoid;

use crate::model::error::DynError;

use super::project::project_root;

pub struct TmpDir {
    path: PathBuf,
}

impl TmpDir {
    pub fn new() -> Result<TmpDir, DynError> {
        let name = nanoid!();
        let dir = project_root().join("tmp").join(name);
        std::fs::create_dir_all(&dir)?;
        Ok(TmpDir { path: dir })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for TmpDir {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.path).ok();
    }
}
